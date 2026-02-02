use clap::{Parser, Subcommand};
use crate::types::Algorithm;

#[derive(Parser)]
#[command(name = "zeta-hash", version, about = "CLI tool for hashing strings and files")]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
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
        #[arg(short, long)]
        verify: Option<String>,
    },
    Salt { length: usize },
    Base {
        #[command(subcommand)]
        cmd: BaseCommands,
    },
}

#[derive(Subcommand)]
pub enum BaseCommands {
    BlockNumber,
    Balance { address: String },
    GasPrice,
    Nonce { address: String },
    TxStatus { hash: String },
    GenerateWallet,
    Send { 
        to: String, 
        amount: String 
    },
    Erc20Balance {
        token: String,
        address: String,
    },
    SendErc20 {
        token: String,
        to: String,
        amount: String,
    },
    Sign {
        message: String,
    },
    Verify {
        message: String,
        signature: String,
        address: String,
    },
    Wrap {
        amount: String,
    },
    Unwrap {
        amount: String,
    },
}