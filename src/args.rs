use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "zeta-hash", version, about = "CLI tool for hashing strings and files")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Algorithm {
    Sha256,
    Keccak256,
    Blake3,
}

#[derive(Subcommand)]
pub enum Commands {
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