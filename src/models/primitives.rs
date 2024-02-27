use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Address([u8; 33]);

impl Address {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut result = [0u8; 33];
        result.copy_from_slice(bytes);
        Address(result)
    }
}

impl Default for Address {
    fn default() -> Self {
        Address([0u8; 33])
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_hex_string())
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Address::from_hex_string(&s))
    }
}

pub type Balance = u128;
pub type Id = u64;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timestamp(u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Signature([u8; 64]);

impl Signature {
    pub fn as_hex_string(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn from_hex_string(s: &str) -> Self {
        let bytes = hex::decode(s).expect("Failed to decode hex string");
        if bytes.len() != 64 {
            panic!("Invalid signature length");
        }
        let mut result = [0u8; 64];
        result.copy_from_slice(&bytes);
        Signature(result)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct H256([u8; 32]);

impl H256 {
    pub fn new(bytes: [u8; 32]) -> Self {
        H256(bytes)
    }

    pub fn zero() -> Self {
        H256([0u8; 32])
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut result = [0u8; 32];
        result.copy_from_slice(slice);
        H256(result)
    }

    pub fn from_hex_string(s: &str) -> Self {
        let bytes = hex::decode(s).expect("Failed to decode hex string");
        if bytes.len() != 32 {
            panic!("Invalid hash length");
        }
        let mut result = [0u8; 32];
        result.copy_from_slice(&bytes);
        H256(result)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn leading_zeros(&self) -> usize {
        self.0.iter().take_while(|&&x| x == 0).count()
    }

    pub fn as_hex_string(&self) -> String {
        hex::encode(&self.0)
    }
}

impl Default for H256 {
    fn default() -> Self {
        H256::zero()
    }
}

impl Serialize for H256 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.as_hex_string())
    }
}

impl<'de> Deserialize<'de> for H256 {
    fn deserialize<D>(deserializer: D) -> Result<H256, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(H256::from_slice(&hex::decode(s).unwrap()))
    }
}

impl Signature {
    pub fn new(bytes: [u8; 64]) -> Self {
        Signature(bytes)
    }

    pub fn from_slice(slice: &[u8]) -> Self {
        let mut result = [0u8; 64];
        result.copy_from_slice(slice);
        Signature(result)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Into<secp256k1::ecdsa::Signature> for Signature {
    fn into(self) -> secp256k1::ecdsa::Signature {
        secp256k1::ecdsa::Signature::from_compact(&self.0).expect("Failed to convert signature")
    }
}

impl From<secp256k1::ecdsa::Signature> for Signature {
    fn from(sig: secp256k1::ecdsa::Signature) -> Self {
        Signature(sig.serialize_compact())
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

impl From<Option<Vec<u8>>> for H256 {
    fn from(bytes: Option<Vec<u8>>) -> Self {
        match bytes {
            Some(bytes) => H256::from_slice(&bytes),
            None => H256::zero(),
        }
    }
}

impl From<secp256k1::PublicKey> for Address {
    fn from(pubkey: secp256k1::PublicKey) -> Self {
        Address(pubkey.serialize())
    }
}

impl Into<secp256k1::PublicKey> for Address {
    fn into(self) -> secp256k1::PublicKey {
        secp256k1::PublicKey::from_slice(&self.0).expect("Failed to convert public key")
    }
}

impl Address {
    pub fn as_hex_string(&self) -> String {
        hex::encode(&self.0)
    }

    pub fn from_hex_string(s: &str) -> Self {
        let bytes = hex::decode(s).expect("Failed to decode hex string");
        if bytes.len() != 33 {
            panic!("Invalid address length {}", bytes.len());
        }
        let mut result = [0u8; 33];
        result.copy_from_slice(&bytes);
        Address(result)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Option<Vec<u8>>> for Address {
    fn from(bytes: Option<Vec<u8>>) -> Self {
        match bytes {
            Some(bytes) => Address::from_bytes(&bytes),
            None => Address::default(),
        }
    }
}
