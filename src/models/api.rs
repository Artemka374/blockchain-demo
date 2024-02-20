use crate::models::merkle_tree::MerkleNode;
use crate::models::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeMode {
    #[default]
    Full,
    Test,
}

impl From<String> for NodeMode {
    fn from(s: String) -> Self {
        match s.as_str() {
            "full" => NodeMode::Full,
            "test" => NodeMode::Test,
            _ => {
                panic!("Invalid node mode: {}", s);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiTransfer {
    pub from: Address,
    pub to: Address,
    pub amount: u128,
    pub nonce: u128,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MineInfo {
    pub miner: Address,
    pub block_nonce: u64,
    pub nonce: u64,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiMint {
    pub to: Address,
    pub amount: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGenerateSig {
    pub message: String,
    pub private_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiVerifySig {
    pub message: String,
    pub signature: String,
    pub public_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiVerifyProof {
    pub tx_hash: String,
    pub proof: Vec<[u8; 33]>,
}
