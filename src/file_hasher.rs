use std::fs::File;
use std::io::{Read, Result};
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};
use blake3;
use hex;

pub struct FileHasher;

impl FileHasher {
    pub fn hash_file_sha256(path: &str) -> Result<String> {
        let mut hasher = Sha256::new();
        Self::process_file(path, |data| hasher.update(data))?;
        Ok(hex::encode(hasher.finalize()))
    }

    pub fn hash_file_keccak256(path: &str) -> Result<String> {
        let mut keccak = Keccak::v256();
        Self::process_file(path, |data| keccak.update(data))?;
        let mut output = [0u8; 32];
        keccak.finalize(&mut output);
        Ok(hex::encode(output))
    }

    pub fn hash_file_blake3(path: &str) -> Result<String> {
        let mut hasher = blake3::Hasher::new();
        Self::process_file(path, |data| { hasher.update(data); })?;
        Ok(hasher.finalize().to_hex().to_string())
    }

    fn process_file<F>(path: &str, mut updater: F) -> Result<()>
    where
        F: FnMut(&[u8]),
    {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; 4096];

        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            updater(&buffer[..n]);
        }
        Ok(())
    }
}