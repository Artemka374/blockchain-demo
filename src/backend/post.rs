use crate::db::{accounts, blocks, transactions, *};
use crate::models::primitives::H256;
use crate::models::{
    api::{ApiMint, ApiTransfer, MineInfo, NodeMode},
    error::ServerError,
    merkle_tree::MerkleTree,
    primitives::Signature,
    TransactionStatus, {Block, Transaction},
};
use crate::NodeData;
use actix_web::{web, HttpResponse};

#[actix_web::post("/add_transaction")]
pub async fn transfer(
    data: web::Data<NodeData>,
    transfer_info: web::Json<ApiTransfer>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = connection(&data.pool).await?;
    let transfer_info = transfer_info.into_inner();

    let mut tx = Transaction {
        hash: H256::zero(),
        from: transfer_info.from,
        to: transfer_info.to,
        amount: transfer_info.amount,
        block_id: None,
        nonce: accounts::get_nonce(&mut conn, transfer_info.from).await?,
        status: TransactionStatus::Pending,
    };

    tx.verify_signature(Signature::from_hex_string(&transfer_info.signature))?;

    tx.hash = tx.hash();

    transactions::add_pending_transaction(&mut conn, tx).await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::post("/try_mine")]
pub async fn try_mine(
    data: web::Data<NodeData>,
    mine_info: web::Json<MineInfo>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = connection(&data.pool).await?;
    let signature = Signature::from_hex_string(&mine_info.signature);

    let transactions =
        transactions::get_pending_transactions(&mut conn, data.config.block_size).await?;

    if transactions.is_empty() {
        return Err(ServerError::new(400, "No transactions to mine".to_string()));
    }

    let mut tree = MerkleTree::new(data.config.merkle_tree_size as usize);

    let tx_hashes = transactions.iter().map(|tx| tx.hash()).collect::<Vec<_>>();

    tree.initialize(tx_hashes.clone())?;

    let (latest_block, latest_hash) = blocks::get_latest_block(&mut conn).await?;

    let mut block = Block {
        id: latest_block + 1,
        hash: None,
        parent_hash: latest_hash,
        merkle_root: tree.root().expect("Merkle tree is empty"),
        nonce: Some(mine_info.nonce),
        produced_by: Some(mine_info.miner),
    };

    let hash = block.compute_hash();

    if data.config.node_mode == NodeMode::Full {
        block.verify(signature)?;

        if hash.leading_zeros() < data.config.target as usize {
            return Err(ServerError::new(
                400,
                "Block does not meet target".to_string(),
            ));
        }
    }

    blocks::add_block(
        &mut conn,
        block.id,
        hash,
        block.parent_hash,
        mine_info.miner,
        mine_info.nonce,
    )
    .await?;

    merkle_tree::add_merkle_tree(&mut conn, block.id, tree).await?;
    transactions::add_transactions_to_block(&mut conn, &tx_hashes, block.id).await?;
    accounts::update_balance(&mut conn, mine_info.miner, data.config.base_reward).await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::post("/set_target")]
pub async fn set_target(
    data: web::Data<NodeData>,
    target: String,
) -> Result<HttpResponse, ServerError> {
    //let mut config = Arc::get_mut(&mut data.into_inner()).unwrap().config;
    //config.target = target;
    let target: u64 = target
        .parse()
        .map_err(|_| ServerError::new(400, "Invalid target".to_string()))?;
    Ok(HttpResponse::Ok().finish())
}

#[actix_web::post("/mint")]
pub async fn mint(
    data: web::Data<NodeData>,
    mint_info: web::Json<ApiMint>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = connection(&data.pool).await?;
    let mint_info = mint_info.into_inner();

    accounts::update_balance(&mut conn, mint_info.to, mint_info.amount).await?;

    Ok(HttpResponse::Ok().finish())
}
