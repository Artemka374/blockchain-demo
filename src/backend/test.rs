use crate::models::api::{ApiGenerateSig, ApiVerifyProof, ApiVerifySig, NodeMode};
use crate::models::error::ServerError;
use crate::models::merkle_tree::{MerkleNode, MerkleProof};
use crate::models::primitives::{Address, Signature, H256};
use crate::{crypto, NodeData};
use actix_web::{web, HttpResponse};

#[actix_web::get("/get_mode")]
pub async fn get_mode(node_data: web::Data<NodeData>) -> Result<HttpResponse, ServerError> {
    Ok(HttpResponse::Ok().json(node_data.config.node_mode.clone()))
}

#[actix_web::post("/set_mode")]
pub async fn set_mode(
    node_data: web::Data<NodeData>,
    mode: web::Json<NodeMode>,
) -> Result<HttpResponse, ServerError> {
    node_data.config.node_mode = mode.into_inner();
    Ok(HttpResponse::Ok().json(node_data.config.node_mode.clone()))
}

#[actix_web::get("/generate_sig/{address}")]
pub async fn generate_sig(
    signature_info: web::Json<ApiGenerateSig>,
) -> Result<HttpResponse, ServerError> {
    let signature_info = signature_info.into_inner();
    let sig = crypto::sig::sign_message(
        &H256::from_slice(signature_info.private_key.as_bytes()),
        &signature_info.message.as_bytes(),
    )?;

    Ok(HttpResponse::Ok().json(sig.as_hex_string()))
}

#[actix_web::get("/verify_sig")]
pub async fn verify_sig(
    verify_sig_info: web::Json<ApiVerifySig>,
) -> Result<HttpResponse, ServerError> {
    let verify_sig_info = verify_sig_info.into_inner();
    let verification_result = crypto::sig::verify_signature(
        &Address::from_hex_string(&verify_sig_info.public_key),
        Signature::from_slice(verify_sig_info.signature.as_bytes()),
        &verify_sig_info.message.as_bytes(),
    )
    .is_ok();

    Ok(HttpResponse::Ok().json(verification_result))
}

#[actix_web::get("/get_pub_key/{private_key}")]
pub async fn get_pub_key(private_key: web::Path<String>) -> Result<HttpResponse, ServerError> {
    let private_key = private_key.into_inner();
    let address = crypto::sig::generate_pub_key(&H256::from_slice(private_key.as_bytes()))
        .map_err(|e| e.into())?;

    Ok(HttpResponse::Ok().json(address.as_hex_string()))
}

#[actix_web::get("/verify_proof")]
pub async fn verify_proof(
    proof_info: web::Json<ApiVerifyProof>,
) -> Result<HttpResponse, ServerError> {
    let proof_info = proof_info.into_inner();
    let hash = H256::from_slice(proof_info.tx_hash.as_bytes());
    let proof = MerkleProof::from_bvtes(proof_info.proof)?;

    Ok(HttpResponse::Ok().json(proof.verify(hash)))
}
