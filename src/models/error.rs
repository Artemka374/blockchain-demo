use std::fmt::format;

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
