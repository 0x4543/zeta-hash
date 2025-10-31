use sha2::{Digest, Sha256};
use tiny_keccak::Keccak;
use blake3;
use hex;

pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn hash_keccak256(input: &str) -> String {
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(input.as_bytes());
    keccak.finalize(&mut output);
    hex::encode(output)
}

pub fn hash_blake3(input: &str) -> String {
    let hash = blake3::hash(input.as_bytes());
    hash.to_hex().to_string()
}
