pub mod types;
pub mod proof;
pub mod difficulty;

use serde::{Deserialize, Serialize};
use std::fmt;

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

/// Mining work unit containing ML operations to perform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkUnit {
    pub id: String,
    pub previous_hash: String,
    pub timestamp: u64,
    pub difficulty: u64,
    pub operations: Vec<MLOperation>,
    pub nonce_range: (u64, u64),
}

/// Machine learning operation types for mining
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MLOperation {
    MatrixMultiply {
        dimensions: (usize, usize, usize), // (m, k, n) for A(m×k) × B(k×n) = C(m×n)
        seed: u64,
    },
    Convolution2D {
        input_shape: (usize, usize, usize, usize), // (batch, channels, height, width)
        kernel_shape: (usize, usize, usize, usize), // (out_ch, in_ch, kh, kw)
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
        shape: (usize, usize, usize, usize), // (batch, channels, height, width)
        epsilon: f32,
        seed: u64,
    },
}

impl MLOperation {
    /// Estimate the number of FP8 operations for this ML operation
    pub fn estimated_flops(&self) -> u64 {
        match self {
            MLOperation::MatrixMultiply { dimensions: (m, k, n), .. } => {
                2 * (*m as u64) * (*k as u64) * (*n as u64)
            }
            MLOperation::Convolution2D {
                input_shape: (batch, _, ih, iw),
                kernel_shape: (out_ch, in_ch, kh, kw),
                stride: (sh, sw),
                ..
            } => {
                let oh = (ih + 2 * 0 - kh) / sh + 1; // assuming no padding for simplicity
                let ow = (iw + 2 * 0 - kw) / sw + 1;
                2 * (*batch as u64) * (*out_ch as u64) * (*in_ch as u64) * 
                    (*kh as u64) * (*kw as u64) * (oh as u64) * (ow as u64)
            }
            MLOperation::MultiHeadAttention {
                batch_size, seq_length, d_model, num_heads, ..
            } => {
                // Simplified FLOPS estimation for attention
                let d_k = d_model / num_heads;
                let qkv_proj = 3 * (*batch_size as u64) * (*seq_length as u64) * (*d_model as u64) * (*d_model as u64);
                let attention = (*batch_size as u64) * (*num_heads as u64) * (*seq_length as u64) * (*seq_length as u64) * (d_k as u64);
                let output_proj = (*batch_size as u64) * (*seq_length as u64) * (*d_model as u64) * (*d_model as u64);
                qkv_proj + 2 * attention + output_proj
            }
            MLOperation::BatchNormalization { shape: (batch, channels, height, width), .. } => {
                // Forward pass: mean, variance, normalize
                6 * (*batch as u64) * (*channels as u64) * (*height as u64) * (*width as u64)
            }
        }
    }
}

impl fmt::Display for MLOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MLOperation::MatrixMultiply { dimensions: (m, k, n), .. } => {
                write!(f, "GEMM({}×{}×{})", m, k, n)
            }
            MLOperation::Convolution2D { input_shape, kernel_shape, .. } => {
                write!(f, "Conv2D({:?}⊛{:?})", input_shape, kernel_shape)
            }
            MLOperation::MultiHeadAttention { seq_length, d_model, num_heads, .. } => {
                write!(f, "MHA(L={}, D={}, H={})", seq_length, d_model, num_heads)
            }
            MLOperation::BatchNormalization { shape, .. } => {
                write!(f, "BatchNorm({:?})", shape)
            }
        }
    }
}

/// Result of executing a work unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkResult {
    pub work_id: String,
    pub nonce: u64,
    pub hash: String,
    pub execution_time_ms: u64,
    pub total_flops: u64,
    pub operation_results: Vec<OperationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationResult {
    pub operation_type: String,
    pub execution_time_ms: u64,
    pub flops: u64,
    pub result_hash: String, // Hash of the output tensor
}

/// Error types for DEMLE operations
#[derive(Debug, thiserror::Error)]
pub enum DemleError {
    #[error("Invalid work unit: {0}")]
    InvalidWorkUnit(String),
    
    #[error("Computation failed: {0}")]
    ComputationError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Proof verification failed: {0}")]
    ProofVerificationError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, DemleError>; 