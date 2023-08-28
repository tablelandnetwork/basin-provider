use once_cell::sync::Lazy;
use primitive_types::H160;
use secp256k1::{
    ecdsa::{RecoverableSignature, RecoveryId},
    All, Message, PublicKey, Secp256k1,
};
use thiserror::Error;
use tiny_keccak::{Hasher, Keccak};

static CONTEXT: Lazy<Secp256k1<All>> = Lazy::new(Secp256k1::new);

/// Error during sender recovery.
#[derive(Error, Debug, PartialEq, Clone)]
pub enum RecoveryError {
    /// A message to recover is invalid. Has to be a non-zero 32-bytes slice.
    #[error("Message has to be a non-zero 32-bytes slice")]
    InvalidMessage,
    /// A signature is invalid and the sender could not be recovered.
    #[error("Signature is invalid (check recovery id)")]
    InvalidSignature,
}

/// Ethereum-like address (right-most 160 bits of a Keccak hash of an ECDSA public key)
pub type Address = H160;

/// Recover a sender, given message and the signature.
pub fn recover(
    message: &[u8],
    signature: &[u8],
    recovery_id: i32,
) -> Result<Address, RecoveryError> {
    let message = Message::from_slice(message).map_err(|_| RecoveryError::InvalidMessage)?;
    let recovery_id =
        RecoveryId::from_i32(recovery_id).map_err(|_| RecoveryError::InvalidSignature)?;
    let signature = RecoverableSignature::from_compact(signature, recovery_id)
        .map_err(|_| RecoveryError::InvalidSignature)?;
    let public_key = CONTEXT
        .recover_ecdsa(&message, &signature)
        .map_err(|_| RecoveryError::InvalidSignature)?;

    Ok(public_key_address(&public_key))
}

/// Gets the address of a public key.
///
/// The public address is defined as the low 20 bytes of the keccak hash of
/// the public key. Note that the public key returned from the `secp256k1`
/// crate is 65 bytes long, that is because it is prefixed by `0x04` to
/// indicate an uncompressed public key; this first byte is ignored when
/// computing the hash.
///
/// Based on https://github.com/tomusdrw/rust-web3/blob/master/src/signing.rs
fn public_key_address(public_key: &PublicKey) -> Address {
    let public_key = public_key.serialize_uncompressed();

    debug_assert_eq!(public_key[0], 0x04);
    let hash = keccak256(&public_key[1..]);

    Address::from_slice(&hash[12..])
}

/// Compute the Keccak-256 hash of input bytes.
pub fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut output = [0u8; 32];
    let mut hasher = Keccak::v256();
    hasher.update(bytes);
    hasher.finalize(&mut output);
    output
}
