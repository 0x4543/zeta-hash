pub mod app;
pub mod args;
pub mod constants;
pub mod error;
pub mod file_hasher;
pub mod random_salt;
pub mod string_hasher;
pub mod types;

pub use app::run;
pub use error::ZetaError;
pub use file_hasher::FileHasher;
pub use random_salt::generate_salt;
pub use string_hasher::{hash_sha256, hash_keccak256, hash_blake3};
pub use types::Algorithm;