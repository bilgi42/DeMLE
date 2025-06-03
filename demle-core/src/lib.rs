pub mod difficulty;
pub mod proof;
pub mod types;

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// DEMLE network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub chain_id: u64,
    pub contract_address: String,
    pub rpc_url: String,
    pub block_time_target: u64, // seconds
    pub initial_difficulty: u64,
    pub difficulty_adjustment_interval: u64, // blocks
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            chain_id: 1337,
            contract_address: "0x0000000000000000000000000000000000000000".to_string(),
            rpc_url: "http://localhost:8545".to_string(),
            block_time_target: 15,
            initial_difficulty: 1_000_000, // 1M teraflops
            difficulty_adjustment_interval: 100,
        }
    }
}

/// Machine learning operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MLOperation {
    MatrixMultiply {
        dimensions: (usize, usize, usize),
        seed: u64,
    },
    Convolution2D {
        input_shape: (usize, usize, usize, usize),
        kernel_shape: (usize, usize, usize, usize),
        stride: (usize, usize),
        padding: (usize, usize),
        seed: u64,
    },
    MultiHeadAttention {
        batch_size: usize,
        seq_length: usize,
        d_model: usize,
        num_heads: usize,
        seed: u64,
    },
    BatchNormalization {
        shape: (usize, usize, usize, usize),
        epsilon: f32,
        seed: u64,
    },
}

impl fmt::Display for MLOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MLOperation::MatrixMultiply { dimensions, .. } => {
                write!(f, "GEMM {}x{}x{}", dimensions.0, dimensions.1, dimensions.2)
            }
            MLOperation::Convolution2D { kernel_shape, .. } => {
                write!(
                    f,
                    "Conv2D {}x{}x{}x{}",
                    kernel_shape.0, kernel_shape.1, kernel_shape.2, kernel_shape.3
                )
            }
            MLOperation::MultiHeadAttention {
                num_heads, d_model, ..
            } => {
                write!(f, "Attention {}heads x {}", num_heads, d_model)
            }
            MLOperation::BatchNormalization { shape, .. } => {
                write!(
                    f,
                    "BatchNorm {}x{}x{}x{}",
                    shape.0, shape.1, shape.2, shape.3
                )
            }
        }
    }
}

/// Result of an ML operation execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub result_hash: String,
    pub flops: u64,
    pub execution_time_ms: u64,
}

/// Work unit for mining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnit {
    pub id: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub difficulty: u64,
    pub operations: Vec<MLOperation>,
    pub nonce_range: (u64, u64),
}

/// Result of mining a work unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkResult {
    pub work_id: String,
    pub nonce: u64,
    pub hash: String,
    pub execution_time_ms: u64,
    pub total_flops: u64,
    pub operation_results: Vec<OperationResult>,
}

/// DEMLE-specific error types
#[derive(Error, Debug)]
pub enum DemleError {
    #[error("Computation error: {0}")]
    ComputationError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

/// Result type for DEMLE operations
pub type Result<T> = std::result::Result<T, DemleError>;
