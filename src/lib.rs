pub mod file_hasher;
pub mod random_salt;

pub use file_hasher::{hash_sha256, hash_keccak256, hash_blake3, FileHasher};
pub use random_salt::generate_salt;