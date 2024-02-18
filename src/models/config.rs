use crate::models::api::NodeMode;

pub struct Config {
    pub server_url: String,
    pub db_url: String,
    pub node_mode: NodeMode,
    pub merkle_tree_size: u64,
    pub base_reward: u64,
    pub block_size: u64,
    pub target: u64,
}

impl Config {
    pub fn parse() -> Self {
        todo!()
    }
}
