pub mod fp8;
pub mod operations;
pub mod gemm;
pub mod convolution;
pub mod attention;
pub mod batch_norm;

use demle_core::{MLOperation, OperationResult, Result};
use std::time::Instant;

pub use fp8::FP8;

/// Execute a machine learning operation and return timing and result information
pub fn execute_ml_operation(operation: &MLOperation) -> Result<OperationResult> {
    let start = Instant::now();
    
    let (result_hash, flops) = match operation {
        MLOperation::MatrixMultiply { dimensions, seed } => {
            gemm::execute_gemm(*dimensions, *seed)?
        }
        MLOperation::Convolution2D { 
            input_shape, 
            kernel_shape, 
            stride, 
            padding, 
            seed 
        } => {
            convolution::execute_conv2d(*input_shape, *kernel_shape, *stride, *padding, *seed)?
        }
        MLOperation::MultiHeadAttention {
            batch_size,
            seq_length,
            d_model,
            num_heads,
            seed,
        } => {
            attention::execute_attention(*batch_size, *seq_length, *d_model, *num_heads, *seed)?
        }
        MLOperation::BatchNormalization {
            shape,
            epsilon,
            seed,
        } => {
            batch_norm::execute_batch_norm(*shape, *epsilon, *seed)?
        }
    };
    
    let execution_time_ms = start.elapsed().as_millis() as u64;
    
    Ok(OperationResult {
        result_hash,
        flops,
        execution_time_ms,
    })
}

/// Execute a complete work unit (sequence of ML operations)
pub fn execute_work_unit(operations: &[MLOperation]) -> Result<Vec<OperationResult>> {
    operations
        .iter()
        .map(execute_ml_operation)
        .collect()
}

/// Calculate total FLOPS from a list of operation results
pub fn calculate_total_flops(results: &[OperationResult]) -> u64 {
    results.iter().map(|r| r.flops).sum()
}

/// Calculate FLOPS per second from results and total time
pub fn calculate_flops_per_second(total_flops: u64, total_time_ms: u64) -> f64 {
    if total_time_ms == 0 {
        0.0
    } else {
        (total_flops as f64) / (total_time_ms as f64 / 1000.0)
    }
}

/// Convert FLOPS to teraflops
pub fn flops_to_teraflops(flops: u64) -> f64 {
    flops as f64 / 1e12
}

/// Convert teraflops to FLOPS
pub fn teraflops_to_flops(teraflops: f64) -> u64 {
    (teraflops * 1e12) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use demle_core::MLOperation;

    #[test]
    fn test_flops_conversion() {
        let teraflops = 2.5;
        let flops = teraflops_to_flops(teraflops);
        let converted_back = flops_to_teraflops(flops);
        
        assert!((converted_back - teraflops).abs() < 0.001);
    }

    #[test]
    fn test_matrix_multiply_execution() {
        let operation = MLOperation::MatrixMultiply {
            dimensions: (32, 32, 32),
            seed: 42,
        };
        
        let result = execute_ml_operation(&operation);
        assert!(result.is_ok());
        
        let result = result.unwrap();
        assert!(result.flops > 0);
        assert!(!result.result_hash.is_empty());
    }

    #[test]
    fn test_work_unit_execution() {
        let operations = vec![
            MLOperation::MatrixMultiply {
                dimensions: (64, 64, 64),
                seed: 1,
            },
            MLOperation::BatchNormalization {
                shape: (32, 64, 32, 32),
                epsilon: 1e-5,
                seed: 2,
            },
        ];
        
        let results = execute_work_unit(&operations).unwrap();
        assert_eq!(results.len(), 2);
        
        let total_flops = calculate_total_flops(&results);
        assert!(total_flops > 0);
    }
} 