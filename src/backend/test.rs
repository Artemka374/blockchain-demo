use crate::models::api::{ApiGenerateSig, ApiVerifyProof, ApiVerifySig, NodeMode};
use crate::models::error::ServerError;
use crate::models::primitives::H256;
use crate::NodeData;
use actix_web::web;

#[actix_web::get("/get_mode")]
pub async fn get_mode(node_data: web::Data<NodeData>) -> Result<NodeMode, ServerError> {
    Ok(node_data.config.node_mode.clone())
}

#[actix_web::post("/set_mode")]
pub async fn set_mode(
    node_data: web::Data<NodeData>,
    mode: web::Json<NodeMode>,
) -> Result<(), ServerError> {
    node_data.config.node_mode = mode.into_inner();
    Ok(())
}

#[actix_web::get("/generate_sig/{address}")]
pub async fn generate_sig(signature_info: web::Json<ApiGenerateSig>) -> Result<H256, ServerError> {
    todo!()
}

#[actix_web::get("/verify_sig")]
pub async fn verify_sig(verify_sig_info: web::Json<ApiVerifySig>) -> Result<bool, ServerError> {
    todo!()
}

#[actix_web::get("/get_pub_key/{private_key}")]
pub async fn get_pub_key(private_key: web::Path<String>) -> Result<H256, ServerError> {
    let private_key = private_key.into_inner();
    todo!()
}

#[actix_web::get("/verify_proof")]
pub async fn verify_proof(proof_info: web::Json<ApiVerifyProof>) -> Result<bool, ServerError> {
    let proof_info = proof_info.into_inner();
    todo!()
}
