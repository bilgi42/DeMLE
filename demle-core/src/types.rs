use serde::{Deserialize, Serialize};

/// Block hash type
pub type BlockHash = String;

/// Transaction hash type  
pub type TxHash = String;

/// Ethereum address type
pub type Address = String;

/// Mining difficulty represented as required teraflops
pub type Difficulty = u64;

/// Mining stats for tracking performance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiningStats {
    pub hashrate: f64,           // Hashes per second
    pub teraflops: f64,          // Teraflops per second
    pub blocks_found: u64,       // Total blocks found
    pub total_operations: u64,   // Total ML operations performed
    pub uptime_seconds: u64,     // Miner uptime
    pub tokens_earned: u64,      // Total DEMLE tokens earned
}

impl Default for MiningStats {
    fn default() -> Self {
        Self {
            hashrate: 0.0,
            teraflops: 0.0,
            blocks_found: 0,
            total_operations: 0,
            uptime_seconds: 0,
            tokens_earned: 0,
        }
    }
} 