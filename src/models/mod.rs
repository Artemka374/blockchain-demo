use crate::crypto::hash::hash_message;
use crate::crypto::sig::verify_signature;
use crate::models::merkle_tree::MerkleTree;
use crate::models::primitives::{Address, Signature, H256};

pub mod api;
pub mod config;
pub mod constants;
pub mod error;
pub mod merkle_tree;
pub mod primitives;

#[derive(Default, Debug, Copy, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Block {
    pub id: u64,
    pub hash: Option<H256>,
    pub parent_hash: H256,
    pub merkle_root: H256,
    pub nonce: Option<u64>,
    pub produced_by: Option<Address>,
}

impl Block {
    pub fn verify(&self, signature: Signature) -> Result<(), error::Error> {
        let message = format!(
            "Mine block miner:{} parent_hash:{} merkle_root:{} nonce:{}",
            self.produced_by.unwrap().as_hex_string(),
            self.parent_hash.as_hex_string(),
            self.merkle_root.as_hex_string(),
            self.nonce.unwrap_or(0)
        );
        verify_signature(
            &self.produced_by.unwrap(),
            signature,
            hash_message(message.as_bytes()).as_slice(),
        )
    }

    pub fn compute_hash(&mut self) -> H256 {
        let message = format!(
            "Block id:{} parent_hash:{} merkle_root:{} nonce:{}",
            self.id,
            self.parent_hash.as_hex_string(),
            self.merkle_root.as_hex_string(),
            self.nonce.unwrap_or(0)
        );
        let hash = hash_message(message.as_bytes());
        self.hash = Some(hash);
        hash
    }
}

#[derive(Default, Debug, Copy, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub nonce: u64,
    pub sig: Signature,
}

impl Transaction {
    pub fn verify_signature(&self) -> Result<(), error::CryptoError> {
        verify_signature(&self.from, self.sig.clone(), &self.hash().as_slice())
    }

    pub fn hash(&self) -> H256 {
        let message = format!(
            "Transfer from:{} to:{} amount:{} nonce:{}",
            self.from.as_hex_string(),
            self.to.as_hex_string(),
            self.amount,
            self.nonce
        );
        hash_message(message.as_bytes())
    }
}
