use crate::models::{
    error::ServerError,
    merkle_tree::MerkleProof,
    Transaction,
    Block
};
}
use actix_web::{web, HttpResponse};
use crate::{db, NodeData};
use crate::models::primitives::{Address, H256};

#[actix_web::get("/get_balance/{address}")]
pub async fn get_balance(data: web::Data<NodeData>, address: web::Path<String>) -> Result<HttpResponse, ServerError> {
    let mut conn = db::connection(&data.pool).await?;
    let address = Address::from_hex_string(&address.into_inner());

    let balance = db::get_balance(&mut conn, address).await?;

    Ok(HttpResponse::Ok().json(balance))
}

#[actix_web::get("/get_transaction/{tx_hash}")]
pub async fn get_transaction(data: web::Data<NodeData>, tx_hash: web::Path<String>) -> Result<HttpResponse, ServerError> {

    let mut conn = db::connection(&data.pool).await?;

    let decoded_hash = hex::decode(tx_hash.into_inner())?;
    let tx_hash = H256::from_slice(&decoded_hash);


    let transaction = db::get_transaction(&mut conn, tx_hash).await?;
    todo!()
}

#[actix_web::get("/get_transactions/{address}")]
pub async fn get_transactions(address: web::Path<String>) -> Result<Vec<Transaction>, ServerError> {
    // address - Address
    let address = address.into_inner();
    todo!()
}

#[actix_web::get("/get_block_by_hash/{block_hash}")]
pub async fn get_block_by_hash(block_hash: web::Path<String>) -> Result<Block, ServerError> {
    // block_hash - H256
    let block_hash = block_hash.into_inner();
    todo!()
}

#[actix_web::get("/get_block_by_id/{block_id}")]
pub async fn get_block_by_id(block_id: web::Path<u64>) -> Result<Block, ServerError> {
    let block_id = block_id.into_inner();
    todo!()
}

#[actix_web::get("/get_proof/{tx_hash}")]
pub async fn get_proof(tx_hash: web::Path<String>) -> Result<MerkleProof, ServerError> {
    // tx_hash - H256
    let tx_hash = tx_hash.into_inner();
    todo!()
}

#[actix_web::get("/get_nonce/{address}")]
pub async fn get_nonce(address: web::Path<String>) -> Result<u64, ServerError> {
    let address = address.into_inner();
    todo!()
}

#[actix_web::get("/get_target")]
pub async fn get_target() -> Result<u64, ServerError> {
    todo!()
}

#[actix_web::get("/block_height")]
pub async fn block_height() -> Result<u64, ServerError> {
    todo!()
}
