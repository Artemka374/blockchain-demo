#[derive(Debug, PartialEq)]
pub enum MerkleTreeError {
    EmptyTree,
    MerkleTreeNotInitialized,
    LeavesAmountGreaterThanTreeSize,
    DeserializingError,
}

pub enum ServerError {
    DatabaseError,
    TransactionError,
    MerkleTreeError(MerkleTreeError),
    CryptoError(CryptoError),
}

impl From<MerkleTreeError> for ServerError {
    fn from(err: MerkleTreeError) -> Self {
        ServerError::MerkleTreeError(err)
    }
}

impl From<CryptoError> for ServerError {
    fn from(err: CryptoError) -> Self {
        ServerError::CryptoError(err)
    }
}

pub enum CryptoError {
    InvalidSignature,
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidMessage,
}
