use ethers::prelude::*;
use std::convert::TryFrom;
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
}