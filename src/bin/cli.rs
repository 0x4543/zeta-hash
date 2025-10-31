use clap::{Parser, Subcommand};
use zeta_hash::{hash_sha256, hash_keccak256, hash_blake3, FileHasher, random_salt::generate_salt};

#[derive(Parser)]
#[command(name = "zeta-hash", version, about = "CLI tool for hashing strings and files")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sha256 { input: String },
    Keccak256 { input: String },
    Blake3 { input: String },
    File { path: String, algo: String },
    Salt { length: usize },
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Sha256 { input } => println!("{}", hash_sha256(&input)),
        Commands::Keccak256 { input } => println!("{}", hash_keccak256(&input)),
        Commands::Blake3 { input } => println!("{}", hash_blake3(&input)),
        Commands::File { path, algo } => {
            let result = match algo.as_str() {
                "sha256" => FileHasher::hash_file_sha256(&path),
                "keccak256" => FileHasher::hash_file_keccak256(&path),
                "blake3" => FileHasher::hash_file_blake3(&path),
                _ => {
                    println!("Unknown algorithm: {}", algo);
                    return;
                }
            };
            match result {
                Ok(h) => println!("{}", h),
                Err(e) => println!("Error: {}", e),
            }
        }
        Commands::Salt { length } => println!("{}", generate_salt(length)),
    }
}