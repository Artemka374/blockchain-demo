use crate::crypto::hash::hash_message;
use crate::crypto::sig::verify_signature;
use crate::models::error::CryptoError;
use crate::models::primitives::{Address, Balance, Id, Signature, H256};
use sqlx::postgres::PgRow;
use sqlx::Row;

pub mod api;
pub mod config;
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
    pub fn verify(&self, signature: Signature) -> Result<(), CryptoError> {
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
            hash_message(message.as_bytes()).as_bytes(),
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

#[derive(Default, Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub enum TransactionStatus {
    #[default]
    Pending,
    Confirmed,
}

impl sqlx::FromRow<'_, PgRow> for TransactionStatus {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(match row.try_get::<&str, _>("status") {
            Ok(s) => s.into(),
            Err(_) => TransactionStatus::Pending,
        })
    }
}

impl TransactionStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionStatus::Pending => "pending",
            TransactionStatus::Confirmed => "confirmed",
        }
    }
}

impl From<&str> for TransactionStatus {
    fn from(s: &str) -> Self {
        match s {
            "pending" => TransactionStatus::Pending,
            "confirmed" => TransactionStatus::Confirmed,
            _ => panic!("Invalid transaction status"),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub struct Transaction {
    pub hash: H256,
    pub from: Address,
    pub to: Address,
    pub amount: Balance,
    pub block_id: Option<Id>,
    pub nonce: u64,
    pub status: TransactionStatus,
}

impl Transaction {
    pub fn verify_signature(&self, signature: Signature) -> Result<(), error::CryptoError> {
        verify_signature(&self.from, signature, &self.hash().as_bytes())
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
