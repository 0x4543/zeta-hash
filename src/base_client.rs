use ethers::prelude::*;
use ethers::utils::{format_ether, format_units};
use std::convert::TryFrom;
use std::str::FromStr;
use std::sync::Arc;
use crate::error::ZetaError;

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
}