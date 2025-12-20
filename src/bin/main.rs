use clap::Parser;
use zeta_hash::args::{Cli, Commands};
use zeta_hash::{hash_sha256, hash_keccak256, hash_blake3, FileHasher, generate_salt, Algorithm};

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli.cmd) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cmd: Commands) -> Result<(), String> {
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
            match result {
                Ok(h) => println!("{}", h),
                Err(e) => return Err(e.to_string()),
            }
        }
        Commands::Salt { length } => println!("{}", generate_salt(length)),
    }
    Ok(())
}