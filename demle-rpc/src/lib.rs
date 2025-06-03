pub mod client;
pub mod contract;

use demle_core::{DemleError, NetworkConfig, Result};
use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, H256, U256};
use web3::Web3;
use sha3::{Digest, Sha3_256};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

/// RPC client for DEMLE blockchain interactions
pub struct DemleRpcClient {
    config: NetworkConfig,
    web3: Web3<web3::transports::Http>,
    contract: Option<Contract<web3::transports::Http>>,
}

impl DemleRpcClient {
    pub fn new(config: NetworkConfig) -> Self {
        let transport = web3::transports::Http::new(&config.rpc_url).unwrap();
        let web3 = web3::Web3::new(transport);
        
        Self {
            config,
            web3,
            contract: None,
        }
    }
    
    /// Initialize the contract instance
    pub async fn init_contract(&mut self) -> Result<()> {
        // Simple DEMLE contract ABI (just the submitMiningProof function)
        let abi = r#"[
            {
                "inputs": [
                    {"internalType": "bytes32", "name": "nonce", "type": "bytes32"},
                    {"internalType": "bytes", "name": "mlProof", "type": "bytes"}
                ],
                "name": "submitMiningProof",
                "outputs": [],
                "stateMutability": "nonpayable",
                "type": "function"
            },
            {
                "inputs": [{"internalType": "address", "name": "account", "type": "address"}],
                "name": "balanceOf",
                "outputs": [{"internalType": "uint256", "name": "", "type": "uint256"}],
                "stateMutability": "view",
                "type": "function"
            }
        ]"#;
        
        let contract_address: Address = self.config.contract_address.parse()
            .map_err(|e| DemleError::ValidationError(format!("Invalid contract address: {}", e)))?;
            
        let contract = Contract::from_json(
            self.web3.eth(),
            contract_address,
            abi.as_bytes(),
        ).map_err(|e| DemleError::NetworkError(format!("Failed to load contract: {}", e)))?;
        
        self.contract = Some(contract);
        Ok(())
    }

    /// Get current block number
    pub async fn get_block_number(&self) -> Result<u64> {
        let block_number = self.web3.eth().block_number().await
            .map_err(|e| DemleError::NetworkError(format!("Failed to get block number: {}", e)))?;
        Ok(block_number.as_u64())
    }

    /// Get current difficulty
    pub async fn get_difficulty(&self) -> Result<u64> {
        // For now, return default difficulty
        Ok(self.config.initial_difficulty)
    }

    /// Submit work result to blockchain
    pub async fn submit_work(&self, work_result: &demle_core::WorkResult) -> Result<String> {
        let contract = self.contract.as_ref()
            .ok_or_else(|| DemleError::NetworkError("Contract not initialized".to_string()))?;
            
        tracing::info!(
            "Submitting work: {} with {} FLOPS",
            work_result.work_id,
            work_result.total_flops
        );

        // Create a unique 32-byte nonce by combining work data with timestamp and random component
        // This ensures every submission is unique for the demo
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let random_value: u64 = rand::thread_rng().gen();
        let nonce_input = format!("{}:{}:{}:{}", work_result.nonce, work_result.work_id, timestamp, random_value);
        let nonce_hash = Sha3_256::digest(nonce_input.as_bytes());
        let nonce = H256::from_slice(&nonce_hash);
        
        // Create ML proof from work result - add some additional verification data
        let proof_data = serde_json::json!({
            "work_id": work_result.work_id,
            "nonce": work_result.nonce,
            "hash": work_result.hash,
            "execution_time_ms": work_result.execution_time_ms,
            "total_flops": work_result.total_flops,
            "operation_results": work_result.operation_results,
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            "verification": "demle_fp8_mining_proof"
        });
        
        let ml_proof = serde_json::to_vec(&proof_data)
            .map_err(|e| DemleError::SerializationError(format!("Failed to serialize proof: {}", e)))?;
        
        tracing::debug!(
            "Submitting nonce: {:?}, proof size: {} bytes",
            nonce,
            ml_proof.len()
        );
        
        // Get accounts (use first account as default)
        let accounts = self.web3.eth().accounts().await
            .map_err(|e| DemleError::NetworkError(format!("Failed to get accounts: {}", e)))?;
            
        let from_account = accounts.first()
            .ok_or_else(|| DemleError::NetworkError("No accounts available".to_string()))?;

        // Submit the transaction
        let tx_hash = contract
            .call("submitMiningProof", (nonce, Bytes::from(ml_proof)), *from_account, Options::default())
            .await
            .map_err(|e| DemleError::NetworkError(format!("Failed to submit transaction: {}", e)))?;

        Ok(format!("{:?}", tx_hash))
    }

    /// Get token balance for an address
    pub async fn get_balance(&self, address: &str) -> Result<u64> {
        let contract = self.contract.as_ref()
            .ok_or_else(|| DemleError::NetworkError("Contract not initialized".to_string()))?;
            
        let addr: Address = address.parse()
            .map_err(|e| DemleError::ValidationError(format!("Invalid address: {}", e)))?;
            
        let balance: U256 = contract
            .query("balanceOf", (addr,), None, Options::default(), None)
            .await
            .map_err(|e| DemleError::NetworkError(format!("Failed to get balance: {}", e)))?;
            
        // Convert from Wei to DEMLE (assuming 18 decimals)
        Ok((balance / U256::from(10).pow(18.into())).as_u64())
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
