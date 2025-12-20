use clap::{Parser, Subcommand, ValueEnum};
use zeta_hash::{hash_sha256, hash_keccak256, hash_blake3, FileHasher, generate_salt};

#[derive(Parser)]
#[command(name = "zeta-hash", version, about = "CLI tool for hashing strings and files")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    Sha256,
    Keccak256,
    Blake3,
}

#[derive(Subcommand)]
enum Commands {
    Sha256 { input: String },
    Keccak256 { input: String },
    Blake3 { input: String },
    File {
        path: String,
        #[arg(value_enum)]
        algo: Algorithm,
    },
    Salt { length: usize },
}

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