use std::time::SystemTime;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Address([u8; 32]);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timestamp(u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct H256([u8; 32]);

impl H256 {
    pub fn new(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut result = [0u8; 32];
        result.copy_from_slice(slice);
        H256(result)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Timestamp {
    pub fn now() -> Self {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Failed to get timestamp");

        Timestamp(time.as_secs())
    }
}
