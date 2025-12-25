use crate::args::Commands;
use crate::error::ZetaError;
use crate::file_hasher::FileHasher;
use crate::random_salt::generate_salt;
use crate::string_hasher::{hash_blake3, hash_keccak256, hash_sha256};
use crate::types::Algorithm;

pub fn run(cmd: Commands) -> Result<(), ZetaError> {
    match cmd {
        Commands::Sha256 { input } => println!("{}", hash_sha256(&input)),
        Commands::Keccak256 { input } => println!("{}", hash_keccak256(&input)),
        Commands::Blake3 { input } => println!("{}", hash_blake3(&input)),
        Commands::File { path, algo } => {
            let result = match algo {
                Algorithm::Sha256 => FileHasher::hash_file_sha256(&path),
                Algorithm::Keccak256 => FileHasher::hash_file_keccak256(&path),
                Algorithm::Blake3 => FileHasher::hash_file_blake3(&path),
            };
            let hash = result?;
            println!("{}", hash);
        }
        Commands::Salt { length } => println!("{}", generate_salt(length)),
    }
    Ok(())
}
