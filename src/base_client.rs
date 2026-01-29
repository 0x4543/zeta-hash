use ethers::prelude::*;
use ethers::signers::{LocalWallet, Signer};
use ethers::utils::{format_ether, format_units, parse_ether, parse_units};
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use crate::error::ZetaError;

abigen!(
    Erc20,
    r#"[
        function balanceOf(address account) external view returns (uint256)
        function decimals() external view returns (uint8)
        function symbol() external view returns (string)
        function transfer(address to, uint256 amount) external returns (bool)
    ]"#
);

pub struct BaseClient {
    provider: Arc<Provider<Http>>,
}

impl BaseClient {
    pub fn new(rpc_url: &str) -> Result<Self, ZetaError> {
        let provider = Provider::<Http>::try_from(rpc_url)
            .map_err(|e| ZetaError::Internal(format!("Invalid RPC URL: {}", e)))?;
        Ok(Self {
            provider: Arc::new(provider),
        })
    }

    pub async fn get_block_number(&self) -> Result<u64, ZetaError> {
        let block = self.provider.get_block_number().await
            .map_err(|e| ZetaError::Internal(format!("Provider error: {}", e)))?;
        Ok(block.as_u64())
    }

    pub async fn get_balance(&self, address: &str) -> Result<String, ZetaError> {
        let addr = Address::from_str(address)
            .map_err(|e| ZetaError::Internal(format!("Invalid address format: {}", e)))?;
        
        let balance = self.provider.get_balance(addr, None).await
            .map_err(|e| ZetaError::Internal(format!("Provider error: {}", e)))?;
            
        Ok(format!("{}", format_ether(balance)))
    }

    pub async fn get_gas_price(&self) -> Result<String, ZetaError> {
        let gas = self.provider.get_gas_price().await
            .map_err(|e| ZetaError::Internal(format!("Provider error: {}", e)))?;
        
        let gwei = format_units(gas, "gwei")
            .map_err(|e| ZetaError::Internal(format!("Format error: {}", e)))?;
            
        Ok(format!("{}", gwei))
    }

    pub async fn get_transaction_count(&self, address: &str) -> Result<u64, ZetaError> {
        let addr = Address::from_str(address)
            .map_err(|e| ZetaError::Internal(format!("Invalid address format: {}", e)))?;

        let nonce = self.provider.get_transaction_count(addr, None).await
            .map_err(|e| ZetaError::Internal(format!("Provider error: {}", e)))?;

        Ok(nonce.as_u64())
    }

    pub async fn get_tx_status(&self, hash: &str) -> Result<String, ZetaError> {
        let tx_hash = H256::from_str(hash)
            .map_err(|e| ZetaError::Internal(format!("Invalid transaction hash: {}", e)))?;

        let receipt = self.provider.get_transaction_receipt(tx_hash).await
            .map_err(|e| ZetaError::Internal(format!("Provider error: {}", e)))?;

        match receipt {
            Some(r) => {
                let status = match r.status {
                    Some(s) if s.as_u64() == 1 => "Success",
                    Some(_) => "Failed",
                    None => "Unknown",
                };
                let gas_used = r.gas_used.unwrap_or_default();
                let block = r.block_number.unwrap_or_default();
                
                Ok(format!(
                    "Status: {}\nBlock: {}\nGas Used: {}", 
                    status, block, gas_used
                ))
            }
            None => Ok("Transaction Pending or Not Found".to_string()),
        }
    }

    pub fn generate_wallet() -> (String, String) {
        let wallet = LocalWallet::new(&mut rand::thread_rng());
        let address = format!("{:?}", wallet.address());
        let priv_key = hex::encode(wallet.signer().to_bytes());
        (address, priv_key)
    }

    pub async fn send_eth(&self, private_key: &str, to: &str, amount_eth: &str) -> Result<String, ZetaError> {
        let chain_id = self.provider.get_chainid().await
            .map_err(|e| ZetaError::Internal(format!("Failed to get chain ID: {}", e)))?;

        let wallet: LocalWallet = private_key.parse::<LocalWallet>()
            .map_err(|e| ZetaError::Internal(format!("Invalid private key: {}", e)))?
            .with_chain_id(chain_id.as_u64());

        let client = SignerMiddleware::new(self.provider.clone(), wallet);

        let to_addr = Address::from_str(to)
            .map_err(|e| ZetaError::Internal(format!("Invalid TO address: {}", e)))?;

        let value = parse_ether(amount_eth)
            .map_err(|e| ZetaError::Internal(format!("Invalid amount: {}", e)))?;

        let tx = TransactionRequest::new()
            .to(to_addr)
            .value(value);

        let pending_tx = client.send_transaction(tx, None).await
            .map_err(|e| ZetaError::Internal(format!("Failed to send transaction: {}", e)))?;

        Ok(format!("{:?}", pending_tx.tx_hash()))
    }

    pub async fn get_erc20_balance(&self, token_address: &str, wallet_address: &str) -> Result<String, ZetaError> {
        let token_addr = Address::from_str(token_address)
            .map_err(|e| ZetaError::Internal(format!("Invalid token address: {}", e)))?;
        
        let wallet_addr = Address::from_str(wallet_address)
            .map_err(|e| ZetaError::Internal(format!("Invalid wallet address: {}", e)))?;

        let contract = Erc20::new(token_addr, self.provider.clone());

        let balance = contract.balance_of(wallet_addr).call().await
            .map_err(|e| ZetaError::Internal(format!("Contract call failed: {}", e)))?;

        let decimals = contract.decimals().call().await
            .unwrap_or(18);

        let symbol = contract.symbol().call().await
            .unwrap_or_else(|_| "TOKEN".to_string());

        let formatted = format_units(balance, decimals as u32)
            .map_err(|e| ZetaError::Internal(format!("Format error: {}", e)))?;

        Ok(format!("{} {}", formatted, symbol))
    }

    pub async fn send_erc20(&self, private_key: &str, token_address: &str, to: &str, amount: &str) -> Result<String, ZetaError> {
        let chain_id = self.provider.get_chainid().await
            .map_err(|e| ZetaError::Internal(format!("Failed to get chain ID: {}", e)))?;

        let wallet: LocalWallet = private_key.parse::<LocalWallet>()
            .map_err(|e| ZetaError::Internal(format!("Invalid private key: {}", e)))?
            .with_chain_id(chain_id.as_u64());

        let client = Arc::new(SignerMiddleware::new(self.provider.clone(), wallet));

        let token_addr = Address::from_str(token_address)
            .map_err(|e| ZetaError::Internal(format!("Invalid token address: {}", e)))?;

        let to_addr = Address::from_str(to)
            .map_err(|e| ZetaError::Internal(format!("Invalid TO address: {}", e)))?;

        let contract = Erc20::new(token_addr, client);

        let decimals = contract.decimals().call().await
            .map_err(|e| ZetaError::Internal(format!("Failed to fetch decimals: {}", e)))?;

        let amount_wei = parse_units(amount, decimals as u32)
            .map_err(|e| ZetaError::Internal(format!("Invalid amount format: {}", e)))?;

        let call = contract.transfer(to_addr, amount_wei);
        let pending_tx = call.send().await
            .map_err(|e| ZetaError::Internal(format!("Failed to send transaction: {}", e)))?;

        let tx_hash = *pending_tx.tx_hash();

        Ok(format!("{:?}", tx_hash))
    }
}