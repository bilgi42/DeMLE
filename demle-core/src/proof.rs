use crate::{DemleError, Result};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

/// Proof of work for ML computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub nonce: u64,
    pub work_hash: String,
    pub operation_hashes: Vec<String>,
    pub total_flops: u64,
    pub timestamp: u64,
}

impl Proof {
    /// Create a new proof from computation results
    pub fn new(
        nonce: u64,
        operation_hashes: Vec<String>,
        total_flops: u64,
        timestamp: u64,
    ) -> Self {
        let combined = format!("{}:{}:{}", nonce, operation_hashes.join(","), total_flops);
        let work_hash = hex::encode(Sha3_256::digest(combined.as_bytes()));

        Self {
            nonce,
            work_hash,
            operation_hashes,
            total_flops,
            timestamp,
        }
    }

    /// Verify that this proof meets the difficulty requirement
    pub fn verify(&self, difficulty: u64) -> Result<bool> {
        // Simple difficulty check: hash must start with certain number of zeros
        let leading_zeros = self.work_hash.chars().take_while(|&c| c == '0').count();

        // Convert difficulty to required leading zeros (simplified)
        let required_zeros = (difficulty as f64).log10() as usize;

        Ok(leading_zeros >= required_zeros)
    }

    /// Calculate the hash of intermediate computation results
    pub fn hash_operation_result(data: &[u8]) -> String {
        hex::encode(Sha3_256::digest(data))
    }
}
