use crate::crypto::hash::hash_nodes;
use crate::models::error::MerkleTreeError;
use crate::models::primitives::H256;
use serde::{Deserialize, Serialize};

pub struct MerkleTree {
    pub size: usize,
    pub depth: usize,
    pub root: Option<H256>,
    pub leaves: Vec<H256>,
    pub nodes: Vec<MerkleNode>,
}

impl MerkleTree {
    pub fn new(size: usize) -> Self {
        let depth = size.checked_ilog2().unwrap();

        if size != 1 << depth {
            panic!("Merkle tree size must be a power of 2");
        }

        MerkleTree {
            size,
            depth: (depth + 1) as usize,
            root: None,
            leaves: Vec::new(),
            nodes: Vec::new(),
        }
    }

    pub fn initialize(&mut self, leaves: Vec<H256>) -> Result<(), MerkleTreeError> {
        self.leaves = leaves.clone();

        self.nodes
            .extend(leaves.into_iter().enumerate().map(|(index, hash)| {
                let direction = if index % 2 == 0 {
                    Some(Direction::Left)
                } else {
                    Some(Direction::Right)
                };
                MerkleNode::new(hash, direction)
            }));

        let mut layer_start = 0;
        let mut layer_end = self.nodes.len();

        for _depth in 1..self.depth {
            for i in (layer_start..layer_end).step_by(2) {
                let left = self.nodes[i].clone();
                let right = self.nodes[i + 1].clone();

                let direction = if (i - layer_start) % 4 < 2 {
                    Some(Direction::Left)
                } else {
                    Some(Direction::Right)
                };

                let node = MerkleNode::from_children(left, right, direction);

                self.nodes.push(node);
            }
            layer_start = layer_end;
            layer_end = self.nodes.len();
        }

        self.nodes.last_mut().unwrap().parent_direction = None;

        self.root = Some(self.nodes.last().unwrap().hash);

        Ok(())
    }

    pub fn root(&self) -> Option<H256> {
        self.root
    }

    pub fn get_proof(&self, leaf_index: usize) -> Result<MerkleProof, MerkleTreeError> {
        let mut proof = MerkleProof::new();

        let mut index = leaf_index;
        let mut layer_size = self.size;

        for i in 0..(self.depth - 1) {
            let sibling_index = index ^ 1;
            let sibling = self.nodes[sibling_index].clone();

            proof.add_node(sibling)?;

            index = layer_size + (leaf_index >> (i + 1));
            layer_size += 1 << (self.depth - i - 2);
        }

        proof.add_node(MerkleNode::new(
            self.root()
                .ok_or(MerkleTreeError::MerkleTreeNotInitialized)?
                .clone(),
            None,
        ))?;

        Ok(proof)
    }
}

#[derive(Clone, Debug)]
pub struct MerkleProof {
    nodes: Vec<MerkleNode>,
}

impl MerkleProof {
    pub fn new() -> Self {
        MerkleProof { nodes: Vec::new() }
    }

    pub fn from_nodes(nodes: Vec<MerkleNode>) -> Self {
        MerkleProof { nodes }
    }

    pub fn get_nodes(&self) -> Vec<MerkleNode> {
        self.nodes.clone()
    }

    pub fn add_node(&mut self, node: MerkleNode) -> Result<(), MerkleTreeError> {
        self.nodes.push(node);
        Ok(())
    }

    pub fn verify(&self, hash: H256) -> bool {
        let mut current_node = MerkleNode::new(hash, None);

        for node in self.nodes.iter() {
            match node.parent_direction {
                Some(Direction::Left) => {
                    current_node = MerkleNode::from_children(node.clone(), current_node, None);
                }
                Some(Direction::Right) => {
                    current_node = MerkleNode::from_children(current_node, node.clone(), None);
                }
                None => {
                    return current_node.hash == node.hash;
                }
            }
        }

        false
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleNode {
    pub hash: H256,
    pub parent_direction: Option<Direction>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Left,
    Right,
}

impl MerkleNode {
    pub fn new(hash: H256, direction: Option<Direction>) -> Self {
        MerkleNode {
            hash,
            parent_direction: direction,
        }
    }

    pub fn from_children(
        left: MerkleNode,
        right: MerkleNode,
        direction: Option<Direction>,
    ) -> Self {
        let hash = hash_nodes(&left.hash, &right.hash);
        MerkleNode {
            hash,
            parent_direction: direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree() {
        let mut tree = MerkleTree::new(8);
        let leaves = vec![
            H256::new([0; 32]),
            H256::new([1; 32]),
            H256::new([2; 32]),
            H256::new([3; 32]),
            H256::new([4; 32]),
            H256::new([5; 32]),
            H256::new([6; 32]),
            H256::new([7; 32]),
        ];

        assert_eq!(tree.depth, 4);
        assert_eq!(tree.size, 8);

        tree.initialize(leaves).expect("Failed to initialize tree");

        let root = tree.root().expect("Failed to get root");

        let proof = tree.get_proof(0).expect("Failed to get proof");

        assert!(proof.verify(H256::new([0; 32])));
        assert!(!proof.verify(H256::new([1; 32])));
    }
}