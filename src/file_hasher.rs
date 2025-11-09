use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};
use blake3;
use hex;
use std::fs::File;
use std::io::{Read, Result};

pub fn hash_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn hash_keccak256(input: &str) -> String {
    let mut keccak = Keccak::v256();
    keccak.update(input.as_bytes());
    let mut output = [0u8; 32];
    keccak.finalize(&mut output);
    hex::encode(output)
}

pub fn hash_blake3(input: &str) -> String {
    let hash = blake3::hash(input.as_bytes());
    hash.to_hex().to_string()
}

pub struct FileHasher;

impl FileHasher {
    pub fn hash_file_sha256(path: &str) -> Result<String> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0u8; 4096];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(hex::encode(hasher.finalize()))
    }

    pub fn hash_file_keccak256(path: &str) -> Result<String> {
        let mut file = File::open(path)?;
        let mut keccak = Keccak::v256();
        let mut buffer = [0u8; 4096];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            keccak.update(&buffer[..n]);
        }

        let mut output = [0u8; 32];
        keccak.finalize(&mut output);
        Ok(hex::encode(output))
    }

    pub fn hash_file_blake3(path: &str) -> Result<String> {
        let mut file = File::open(path)?;
        let mut hasher = blake3::Hasher::new();
        let mut buffer = [0u8; 4096];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            hasher.update(&buffer[..n]);
        }

        Ok(hasher.finalize().to_hex().to_string())
    }
}