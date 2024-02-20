use crate::db::blocks;
use crate::models::primitives::{Address, H256};
use crate::models::{
    error::ServerError, merkle_tree, merkle_tree::MerkleProof, Block, Transaction,
};
use crate::{db, NodeData};
use actix_web::{web, HttpResponse};
use db::{accounts, transactions};

#[actix_web::get("/get_balance/{address}")]
pub async fn get_balance(
    data: web::Data<NodeData>,
    address: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&data.pool).await?;
    let address = Address::from_hex_string(&address.into_inner());

    let balance = accounts::get_balance(&mut conn, address).await?;

    Ok(HttpResponse::Ok().json(balance))
}

#[actix_web::get("/get_transaction/{tx_hash}")]
pub async fn get_transaction(
    data: web::Data<NodeData>,
    tx_hash: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&data.pool).await?;

    let decoded_hash = hex::decode(tx_hash.into_inner())?;
    let tx_hash = H256::from_slice(&decoded_hash);

    let tx = transactions::get_transaction(&mut conn, tx_hash).await?;
    Ok(HttpResponse::Ok().json(tx))
}

#[actix_web::get("/get_transactions/{address}")]
pub async fn get_transactions(
    data: web::Data<NodeData>,
    address: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    let address = Address::from_hex_string(&address.into_inner());
    let mut conn = db::connection(&data.pool).await?;

    let txs = transactions::get_transactions(&mut conn, address).await?;

    Ok(HttpResponse::Ok().json(txs))
}

#[actix_web::get("/get_block_by_hash/{block_hash}")]
pub async fn get_block_by_hash(
    data: web::Data<NodeData>,
    block_hash: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    let block_hash = H256::from_slice(block_hash.into_inner().as_bytes());
    let mut conn = db::connection(&data.pool).await?;

    let block = blocks::get_block_by_hash(&mut conn, block_hash).await?;

    Ok(HttpResponse::Ok().json(block))
}

#[actix_web::get("/get_block_by_id/{block_id}")]
pub async fn get_block_by_id(
    data: web::Data<NodeData>,
    block_id: web::Path<u64>,
) -> Result<HttpResponse, ServerError> {
    let block_id = block_id.into_inner();
    let mut conn = db::connection(&data.pool).await?;

    let block = blocks::get_block_by_id(&mut conn, block_id).await?;

    Ok(HttpResponse::Ok().json(block))
}

#[actix_web::get("/get_proof/{tx_hash}")]
pub async fn get_proof(
    data: web::Data<NodeData>,
    tx_hash: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    let tx_hash = H256::from_slice(tx_hash.into_inner().as_bytes());

    let mut conn = db::connection(&data.pool).await?;

    let (index, block_id) = db::merkle_tree::get_transaction_index_and_block(&mut conn, tx_hash)
        .await?
        .ok_or(ServerError::new(
            404,
            format!(
                "Transaction with hash {} not found",
                tx_hash.as_hex_string()
            ),
        ))?;

    let tree = db::merkle_tree::get_merkle_tree(&mut conn, block_id).await?;
    let proof = tree.get_proof(index)?;

    let encoded_proof = proof.as_bvtes().iter().map(|b| hex::encode(b)).collect();
    Ok(HttpResponse::Ok().json(encoded_proof))
}

#[actix_web::get("/get_nonce/{address}")]
pub async fn get_nonce(
    data: web::Data<NodeData>,
    address: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&data.pool).await?;
    let address = Address::from_hex_string(&address.into_inner());

    let nonce = accounts::get_nonce(&mut conn, address).await?;

    Ok(HttpResponse::Ok().json(nonce))
}

#[actix_web::get("/get_target")]
pub async fn get_target(data: web::Data<NodeData>) -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().json(data.config.target))
}

#[actix_web::get("/block_height")]
pub async fn block_height(data: web::Data<NodeData>) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&data.pool).await?;

    let (block_id, _) = db::blocks::get_latest_block(&mut conn).await?;

    Ok(HttpResponse::Ok().json(block_id))
}
