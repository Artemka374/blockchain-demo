use crate::models::primitives::H256;
use blake2::Digest;

pub fn hash_nodes(left: &H256, right: &H256) -> H256 {
    let mut hasher = blake2::Blake2s256::new();

    let data = left
        .as_bytes()
        .into_iter()
        .chain(right.as_bytes().into_iter())
        .cloned()
        .collect::<Vec<u8>>();

    hasher.update(&data);

    let result = hasher.finalize();

    H256::from_slice(&result[..])
}

pub fn hash_message(msg: &[u8]) -> H256 {
    let mut hasher = blake2::Blake2s256::new();

    hasher.update(msg);

    let result = hasher.finalize();

    H256::from_slice(&result[..])
}
