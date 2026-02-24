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
            match cmd {
                BaseCommands::GenerateWallet => {
                    let (addr, pk) = BaseClient::generate_wallet();
                    println!("New Wallet Generated:");
                    println!("Address:     {}", addr);
                    println!("Private Key: {}", pk);
                    println!("WARNING: Save this private key securely.");
                }
                BaseCommands::Sign { message } => {
                     let pk = env::var("BASE_PRIVATE_KEY")
                        .map_err(|_| ZetaError::Internal("BASE_PRIVATE_KEY env var not found".to_string()))?;
                    
                    let sig = BaseClient::sign_message(&pk, &message).await?;
                    println!("Message: {}", message);
                    println!("Signature: {}", sig);
                }
                BaseCommands::Verify { message, signature, address } => {
                    let is_valid = BaseClient::verify_signature(&message, &signature, &address)?;
                    if is_valid {
                        println!("Signature Valid: YES");
                    } else {
                        println!("Signature Valid: NO");
                    }
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
                        BaseCommands::BlockInfo { number } => {
                            let info = client.get_block_info(number).await?;
                            println!("{}", info);
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
                            println!("Nonce: {}", nonce);
                        }
                        BaseCommands::TxStatus { hash } => {
                            let status = client.get_tx_status(&hash).await?;
                            println!("{}", status);
                        }
                        BaseCommands::Send { to, amount } => {
                            let pk = env::var("BASE_PRIVATE_KEY")
                                .map_err(|_| ZetaError::Internal("BASE_PRIVATE_KEY env var not found".to_string()))?;
                            
                            println!("Sending {} ETH to {}...", amount, to);
                            let tx_hash = client.send_eth(&pk, &to, &amount).await?;
                            println!("Transaction Sent! Hash: {}", tx_hash);
                        }
                        BaseCommands::Erc20Balance { token, address } => {
                            let balance = client.get_erc20_balance(&token, &address).await?;
                            println!("Balance: {}", balance);
                        }
                        BaseCommands::SendErc20 { token, to, amount } => {
                            let pk = env::var("BASE_PRIVATE_KEY")
                                .map_err(|_| ZetaError::Internal("BASE_PRIVATE_KEY env var not found".to_string()))?;

                            println!("Sending {} tokens to {}...", amount, to);
                            let tx_hash = client.send_erc20(&pk, &token, &to, &amount).await?;
                            println!("Transaction Sent! Hash: {}", tx_hash);
                        }
                        BaseCommands::Wrap { amount } => {
                            let pk = env::var("BASE_PRIVATE_KEY")
                                .map_err(|_| ZetaError::Internal("BASE_PRIVATE_KEY env var not found".to_string()))?;

                            println!("Wrapping {} ETH to WETH...", amount);
                            let tx_hash = client.wrap_eth(&pk, &amount).await?;
                            println!("Transaction Sent! Hash: {}", tx_hash);
                        }
                        BaseCommands::Unwrap { amount } => {
                            let pk = env::var("BASE_PRIVATE_KEY")
                                .map_err(|_| ZetaError::Internal("BASE_PRIVATE_KEY env var not found".to_string()))?;

                            println!("Unwrapping {} WETH to ETH...", amount);
                            let tx_hash = client.unwrap_eth(&pk, &amount).await?;
                            println!("Transaction Sent! Hash: {}", tx_hash);
                        }
                        BaseCommands::GenerateWallet | 
                        BaseCommands::Sign { .. } | 
                        BaseCommands::Verify { .. } => unreachable!(),
                    }
                }
            }
        }
    }
    Ok(())
}