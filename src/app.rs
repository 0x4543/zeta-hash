use std::env;
use crate::args::{BaseCommands, Commands};
use crate::base_client::BaseClient;
use crate::error::ZetaError;
use crate::file_hasher::FileHasher;
use crate::random_salt::generate_salt;
use crate::string_hasher::{hash_blake3, hash_keccak256, hash_sha256};
use crate::types::Algorithm;

pub async fn run(cmd: Commands) -> Result<(), ZetaError> {
    match cmd {
        Commands::Sha256 { input } => println!("{}", hash_sha256(&input)),
        Commands::Keccak256 { input } => println!("{}", hash_keccak256(&input)),
        Commands::Blake3 { input } => println!("{}", hash_blake3(&input)),
        Commands::File { path, algo, verify } => {
            let result = match algo {
                Algorithm::Sha256 => FileHasher::hash_file_sha256(&path),
                Algorithm::Keccak256 => FileHasher::hash_file_keccak256(&path),
                Algorithm::Blake3 => FileHasher::hash_file_blake3(&path),
            };
            let hash = result?;

            if let Some(expected) = verify {
                if hash.eq_ignore_ascii_case(&expected) {
                    println!("Verified: OK");
                } else {
                    eprintln!("Verified: FAILED");
                    eprintln!("Expected: {}", expected);
                    eprintln!("Actual:   {}", hash);
                    return Err(ZetaError::Internal("Hash verification failed".to_string()));
                }
            } else {
                println!("{}", hash);
            }
        }
        Commands::Salt { length } => println!("{}", generate_salt(length)),
        Commands::Base { cmd } => {
            // Для генерации кошелька RPC не нужен, но для других команд нужен
            // Создаем клиент лениво или обрабатываем исключение, если RPC не нужен?
            // Проще всего создать клиента, но если URL кривой, generate_wallet не сработает.
            // Перенесем логику создания клиента ВНУТРЬ веток, где он нужен.
            
            match cmd {
                BaseCommands::GenerateWallet => {
                    let (addr, pk) = BaseClient::generate_wallet();
                    println!("New Wallet Generated:");
                    println!("Address:     {}", addr);
                    println!("Private Key: {}", pk);
                    println!("WARNING: Save this private key securely. It cannot be recovered.");
                }
                _ => {
                    let rpc_url = env::var("BASE_RPC_URL")
                        .unwrap_or_else(|_| "https://mainnet.base.org".to_string());
                    let client = BaseClient::new(&rpc_url)?;

                    match cmd {
                        BaseCommands::BlockNumber => {
                            let bn = client.get_block_number().await?;
                            println!("Base Mainnet Block: {}", bn);
                        }
                        BaseCommands::Balance { address } => {
                            let balance = client.get_balance(&address).await?;
                            println!("Balance: {} ETH", balance);
                        }
                        BaseCommands::GasPrice => {
                            let gas = client.get_gas_price().await?;
                            println!("Current Gas Price: {} Gwei", gas);
                        }
                        BaseCommands::Nonce { address } => {
                            let nonce = client.get_transaction_count(&address).await?;
                            println!("Nonce (Transaction Count): {}", nonce);
                        }
                        BaseCommands::TxStatus { hash } => {
                            let status = client.get_tx_status(&hash).await?;
                            println!("{}", status);
                        }
                        BaseCommands::GenerateWallet => unreachable!(),
                    }
                }
            }
        }
    }
    Ok(())
}