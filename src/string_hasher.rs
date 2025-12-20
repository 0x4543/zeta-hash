use sha2::{Digest, Sha256};
use tiny_keccak::{Hasher, Keccak};
use blake3;
use hex;

pub fn hash_sha256<D: AsRef<[u8]>>(input: D) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_ref());
    hex::encode(hasher.finalize())
}

pub fn hash_keccak256<D: AsRef<[u8]>>(input: D) -> String {
    let mut keccak = Keccak::v256();
    keccak.update(input.as_ref());
    let mut output = [0u8; 32];
    keccak.finalize(&mut output);
    hex::encode(output)
}

pub fn hash_blake3<D: AsRef<[u8]>>(input: D) -> String {
    let hash = blake3::hash(input.as_ref());
    hash.to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hello() {
        let input = "hello";
        let expected = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        assert_eq!(hash_sha256(input), expected);
    }

    #[test]
    fn test_keccak256_hello() {
        let input = "hello";
        let expected = "1c8aff950685c2ed4bc3174f3472287b56d9517b9c948127319a09a7a36deac8";
        assert_eq!(hash_keccak256(input), expected);
    }

    #[test]
    fn test_blake3_hello() {
        let input = "hello";
        let expected = "ea8f163db29966664e8e8e396379d788963050c8227092955f106460e56317bc";
        assert_eq!(hash_blake3(input), expected);
    }

    #[test]
    fn test_bytes_input() {
        let input = b"hello";
        let expected = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
        assert_eq!(hash_sha256(input), expected);
    }
}