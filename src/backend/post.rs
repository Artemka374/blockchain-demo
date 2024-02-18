use crate::crypto::hash::hash_message;
use crate::crypto::sig::verify_signature;
use crate::models::api::{ApiMint, ApiTransfer, MineInfo, NodeMode};
use crate::models::error::ServerError;
use crate::models::merkle_tree::MerkleTree;
use crate::models::primitives::Signature;
use crate::models::{Block, Transaction};
use crate::{db, NodeData};
use actix_web::web;

#[actix_web::post("/add_transaction")]
pub async fn transfer(
    data: web::Data<NodeData>,
    transfer_info: web::Json<ApiTransfer>,
) -> Result<(), ServerError> {
    let mut conn = db::connection(&data.pool).await?;
    let transfer_info = transfer_info.into_inner();

    let tx = Transaction {
        from: transfer_info.from,
        to: transfer_info.to,
        amount: transfer_info.amount as u64,
        nonce: db::get_nonce(&mut conn, transfer_info.from).await?,
        sig: Signature::from_slice(transfer_info.signature.as_bytes()),
    };

    tx.verify_signature()?;

    db::add_pending_transaction(&mut conn, tx).await?;

    Ok(())
}

#[actix_web::post("/try_mine")]
pub async fn try_mine(
    data: web::Data<NodeData>,
    mine_info: web::Json<MineInfo>,
) -> Result<(), ServerError> {
    let mut conn = db::connection(&data.pool).await?;
    let signature = Signature::from_slice(mine_info.signature.as_bytes());

    let transactions = db::get_pending_transactions(&mut conn, data.config.block_size).await?;

    if transactions.is_empty() {
        return Err(ServerError::new(400, "No transactions to mine".to_string()));
    }

    let mut tree = MerkleTree::new(data.config.merkle_tree_size as usize);

    let tx_hashes = transactions.iter().map(|tx| tx.hash()).collect::<Vec<_>>();

    tree.initialize(tx_hashes.clone())?;

    let (latest_block, latest_hash) = db::get_latest_block(&mut conn).await?;

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

        if hash.leading_zeros() < data.config.target {
            return Err(ServerError::new(
                400,
                "Block does not meet target".to_string(),
            ));
        }
    }

    db::add_block(
        &mut conn,
        block.id,
        hash,
        block.parent_hash,
        mine_info.miner,
    )
    .await?;
    //todo update method
    db::add_merkle_tree(&mut conn).await?;
    db::add_transactions_to_block(&mut conn, &tx_hashes, block.id).await?;
    db::update_balance(&mut conn, mine_info.miner, data.config.base_reward).await?;

    Ok(())
}

#[actix_web::post("/set_target")]
pub async fn set_target(data: web::Data<NodeData>, target: u64) -> Result<(), ServerError> {
    data.config.target = target;
    Ok(())
}

#[actix_web::post("/mint")]
pub async fn mint(
    data: web::Data<NodeData>,
    mint_info: web::Json<ApiMint>,
) -> Result<(), ServerError> {
    let mut conn = db::connection(&data.pool).await?;
    let mint_info = mint_info.into_inner();

    db::update_balance(&mut conn, mint_info.to, mint_info.amount as i64).await?;

    Ok(())
}
