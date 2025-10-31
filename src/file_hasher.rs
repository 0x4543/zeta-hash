use std::fs::File;
use std::io::{Read, Result};
use sha2::{Digest, Sha256};
use tiny_keccak::Keccak;
use blake3;
use hex;

pub fn hash_file_sha256(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut hasher = Sha256::new();
    hasher.update(&buffer);
    Ok(hex::encode(hasher.finalize()))
}

pub fn hash_file_keccak256(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(&buffer);
    keccak.finalize(&mut output);
    Ok(hex::encode(output))
}

pub fn hash_file_blake3(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let hash = blake3::hash(&buffer);
    Ok(hash.to_hex().to_string())
}