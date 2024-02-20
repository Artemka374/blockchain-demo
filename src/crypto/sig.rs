use crate::models::error::CryptoError;
use crate::models::primitives;
use crate::models::primitives::{Address, H256};
use secp256k1::Secp256k1;

/// Verifies the signature of a message
pub fn verify_signature(
    pubkey: &Address,
    sig: primitives::Signature,
    msg: &[u8],
) -> Result<(), CryptoError> {
    let secp = Secp256k1::new();
    let message =
        secp256k1::Message::from_digest_slice(msg).map_err(|_| CryptoError::InvalidMessage)?;

    secp.verify_ecdsa(&message, &sig.into(), &pubkey.into())
        .map_err(|_| CryptoError::InvalidSignature)
}

/// Signs a message with a private key
pub fn sign_message(private_key: &H256, msg: &[u8]) -> Result<primitives::Signature, CryptoError> {
    let secp = Secp256k1::new();
    let message =
        secp256k1::Message::from_digest_slice(msg).map_err(|_| CryptoError::InvalidMessage)?;
    let secret_key = secp256k1::SecretKey::from_slice(&private_key.as_bytes())
        .map_err(|_| CryptoError::InvalidPrivateKey)?;

    let sig = secp.sign_ecdsa(&message, &secret_key);
    Ok(primitives::Signature::from(sig))
}

pub fn generate_pub_key(private_key: &H256) -> Result<Address, CryptoError> {
    let secp = Secp256k1::new();
    let secret_key = secp256k1::SecretKey::from_slice(&private_key.as_bytes())
        .map_err(|_| CryptoError::InvalidPrivateKey)?;

    Ok(Address::from(secp256k1::PublicKey::from_secret_key(
        &secp,
        &secret_key,
    )))
}
