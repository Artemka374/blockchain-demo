use crate::models::api::NodeMode;
use crate::models::primitives::Balance;

pub struct Config {
    pub server_url: String,
    pub db_url: String,
    pub node_mode: NodeMode,
    pub merkle_tree_size: u64,
    pub base_reward: Balance,
    pub block_size: u64,
    pub target: u64,
}

impl Config {
    pub fn parse() -> Self {
        todo!()
    }
}
