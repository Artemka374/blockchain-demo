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
}

pub enum CryptoError {
    InvalidSignature,
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidMessage,
}
