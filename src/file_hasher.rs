use std::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};
use blake3;
use hex;
use crate::constants::BUFFER_SIZE;

pub struct FileHasher;

impl FileHasher {
    pub fn hash_file_sha256<P: AsRef<Path>>(path: P) -> Result<String> {
        let mut hasher = Sha256::new();
        Self::process_file(path, |data| hasher.update(data))?;
        Ok(hex::encode(hasher.finalize()))
    }

    pub fn hash_file_keccak256<P: AsRef<Path>>(path: P) -> Result<String> {
        let mut keccak = Keccak::v256();
        Self::process_file(path, |data| keccak.update(data))?;
        let mut output = [0u8; 32];
        keccak.finalize(&mut output);
        Ok(hex::encode(output))
    }

    pub fn hash_file_blake3<P: AsRef<Path>>(path: P) -> Result<String> {
        let mut hasher = blake3::Hasher::new();
        Self::process_file(path, |data| { hasher.update(data); })?;
        Ok(hasher.finalize().to_hex().to_string())
    }

    fn process_file<P: AsRef<Path>, F>(path: P, mut updater: F) -> Result<()>
    where
        F: FnMut(&[u8]),
    {
        let mut file = File::open(path)?;
        let mut buffer = [0u8; BUFFER_SIZE];

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_file_hashing_flow() {
        let path = "test_temp_file.txt";
        let content = b"zeta-hash-test-content";

        {
            let mut file = File::create(path).unwrap();
            file.write_all(content).unwrap();
        }

        let sha256 = FileHasher::hash_file_sha256(path);
        let keccak = FileHasher::hash_file_keccak256(path);
        let blake3 = FileHasher::hash_file_blake3(path);

        let _ = std::fs::remove_file(path);

        assert!(sha256.is_ok());
        assert!(keccak.is_ok());
        assert!(blake3.is_ok());
    }
}