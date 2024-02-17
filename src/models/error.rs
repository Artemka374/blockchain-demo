#[derive(Debug, PartialEq)]
pub enum MerkleTreeError {
    EmptyTree,
    MerkleTreeNotInitialized,
}

pub enum ServerError {
    DatabaseError,
    TransactionError,
    MerkleTreeError(MerkleTreeError),
}
