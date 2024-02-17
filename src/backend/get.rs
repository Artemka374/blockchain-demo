use crate::models::error::ServerError;
use crate::models::primitives::{Address, H256};
use crate::models::{BlockHeader, Transaction};
use actix_web::{web, HttpResponse};

#[actix_web::get("/get_balance/{address}")]
pub async fn get_balance(address: web::Path<String>) -> Result<HttpResponse, ServerError> {
    todo!()
}
#[actix_web::get("/get_transaction/{tx_hash}")]
pub async fn get_transaction(tx_hash: web::Path<String>) -> Result<Transaction, ServerError> {
    // tx_hash - H256
    let tx_hash = tx_hash.into_inner();

    todo!()
}

#[actix_web::get("/get_transactions/{address}")]
pub async fn get_transactions(address: web::Path<String>) -> Result<Vec<Transaction>, ServerError> {
    // address - Address
    let address = address.into_inner();
    todo!()
}

#[actix_web::get("/get_block_by_hash/{block_hash}")]
pub async fn get_block_by_hash(block_hash: web::Path<String>) -> Result<BlockHeader, ServerError> {
    // block_hash - H256
    let block_hash = block_hash.into_inner();
    todo!()
}

#[actix_web::get("/get_block_by_id/{block_id}")]
pub async fn get_block_by_id(block_id: web::Path<u64>) -> Result<BlockHeader, ServerError> {
    let block_id = block_id.into_inner();
    todo!()
}

#[actix_web::get("/get_proof/{tx_hash}")]
pub async fn get_proof(tx_hash: web::Path<String>) -> Result<MerkleProof, ServerError> {
    // tx_hash - H256
    let tx_hash = tx_hash.into_inner();
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
