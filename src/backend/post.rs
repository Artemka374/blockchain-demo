use crate::models::api::{ApiMint, ApiTransfer, MineInfo};
use crate::models::error::ServerError;
use crate::models::Transaction;
use actix_web::web;

#[actix_web::post("/add_transaction")]
pub async fn transfer(transfer_info: web::Json<ApiTransfer>) -> Result<(), ServerError> {
    todo!()
}

#[actix_web::post("/try_mine")]
pub async fn try_mine(mine_info: web::Json<MineInfo>) -> Result<(), ServerError> {
    todo!()
}

#[actix_web::post("/set_target")]
pub async fn set_target(target: u64) -> Result<(), ServerError> {
    todo!()
}

#[actix_web::post("/mint")]
pub async fn mint(mint_info: web::Json<ApiMint>) -> Result<(), ServerError> {
    todo!()
}
