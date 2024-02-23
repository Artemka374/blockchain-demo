use crate::models::api::NodeMode;
use crate::models::primitives::Balance;
use std::env;

#[derive(Debug, Clone)]
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
        let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let node_mode = NodeMode::from(env::var("NODE_MODE").expect("NODE_MODE must be set"));
        let merkle_tree_size = env::var("MERKLE_TREE_SIZE")
            .expect("MERKLE_TREE_SIZE must be set")
            .parse()
            .expect("MERKLE_TREE_SIZE must be a number");
        let base_reward = env::var("BASE_REWARD")
            .expect("BASE_REWARD must be set")
            .parse()
            .expect("MERKLE_TREE_SIZE must be a number");
        let block_size = env::var("BLOCK_SIZE")
            .expect("BLOCK_SIZE must be set")
            .parse()
            .expect("BLOCK_SIZE must be a number");
        let target = env::var("TARGET")
            .expect("TARGET must be set")
            .parse()
            .expect("TARGET must be a number");

        Self {
            server_url,
            db_url,
            node_mode,
            merkle_tree_size,
            base_reward,
            block_size,
            target,
        }
    }
}
