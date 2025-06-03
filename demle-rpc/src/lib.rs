pub mod client;
pub mod contract;

use demle_core::{Result, DemleError, NetworkConfig};

/// RPC client for DEMLE blockchain interactions
pub struct DemleRpcClient {
    config: NetworkConfig,
    http_client: reqwest::Client,
}

impl DemleRpcClient {
    pub fn new(config: NetworkConfig) -> Self {
        Self {
            config,
            http_client: reqwest::Client::new(),
        }
    }
    
    /// Get current block number
    pub async fn get_block_number(&self) -> Result<u64> {
        // Placeholder implementation
        Ok(12345)
    }
    
    /// Get current difficulty
    pub async fn get_difficulty(&self) -> Result<u64> {
        // Placeholder implementation
        Ok(self.config.initial_difficulty)
    }
    
    /// Submit work result to blockchain
    pub async fn submit_work(&self, work_result: &demle_core::WorkResult) -> Result<String> {
        tracing::info!("Submitting work: {} with {} FLOPS", 
                      work_result.work_id, work_result.total_flops);
        
        // Placeholder: return transaction hash
        Ok(format!("0x{:x}", md5::compute(&work_result.hash)))
    }
    
    /// Get token balance for an address
    pub async fn get_balance(&self, address: &str) -> Result<u64> {
        // Placeholder implementation
        Ok(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rpc_client_creation() {
        let config = NetworkConfig::default();
        let client = DemleRpcClient::new(config);
        
        let block_number = client.get_block_number().await.unwrap();
        assert!(block_number > 0);
    }
} 