use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MerkleTreeError {
    EmptyTree,
    MerkleTreeNotInitialized,
    LeavesAmountGreaterThanTreeSize,
    DeserializingError,
}

#[derive(Debug, PartialEq)]
pub enum CryptoError {
    InvalidSignature,
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidMessage,
}

#[derive(Debug, PartialEq)]
pub struct ServerError {
    pub code: u16,
    pub message: String,
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR);

        let error_message = match self.code < 500 {
            true => self.message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": error_message }))
    }
}

impl ServerError {
    pub fn new(code: u16, message: String) -> Self {
        ServerError { code, message }
    }
}

impl From<MerkleTreeError> for ServerError {
    fn from(err: MerkleTreeError) -> Self {
        ServerError::new(400, format!("MerkleTreeError: {:?}", err))
    }
}

impl From<CryptoError> for ServerError {
    fn from(err: CryptoError) -> Self {
        ServerError::new(400, format!("Crypto error: {:?}", err))
    }
}
