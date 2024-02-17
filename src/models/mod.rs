use crate::models::primitives::H256;

pub mod api;
pub mod error;
pub mod merkle_tree;
pub mod primitives;

pub struct Block;

impl Block {
    pub fn new(parent_hash: H256, transactions: Vec<Transaction>) -> Self {
        Block
    }

    pub fn compute_with_nonce(&self, nonce: u64) -> Result<(), error::Error> {
        todo!();
        Ok(())
    }

    fn compute_hash(&mut self) -> Result<(), error::Error> {
        todo!();
        Ok(())
    }

    fn verify(&self) -> Result<(), error::Error> {
        todo!();
        Ok(())
    }
}

pub struct BlockHeader;

pub struct Transaction {
    from: primitives::Address,
    to: primitives::Address,
    amount: u64,
    nonce: u64,
    sig: [u8; 64],
}

impl Transaction {
    pub fn verify_signature(&self) -> Result<(), error::Error> {
        todo!();
        Ok(())
    }
}
