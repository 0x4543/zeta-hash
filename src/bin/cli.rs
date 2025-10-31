use clap::{Parser, Subcommand};
use zeta_hash::{hash_sha256, hash_keccak256, hash_blake3};

#[derive(Parser)]
#[command(name = "zeta-hash", version, about = "CLI tool for hashing strings")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Sha256 { input: String },
    Keccak256 { input: String },
    Blake3 { input: String },
}

fn main() {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Sha256 { input } => println!("{}", hash_sha256(&input)),
        Commands::Keccak256 { input } => println!("{}", hash_keccak256(&input)),
        Commands::Blake3 { input } => println!("{}", hash_blake3(&input)),
    }
}
