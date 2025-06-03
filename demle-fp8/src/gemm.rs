use crate::fp8::FP8;
use demle_core::{proof::Proof, DemleError, Result};
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rayon::prelude::*;

#[cfg(feature = "cuda")]
use candle_core::{Device, Tensor};

/// Execute FP8 GEMM operation: C = A * B
/// Uses GPU acceleration when available
pub fn execute_gemm(dimensions: (usize, usize, usize), seed: u64) -> Result<(String, u64)> {
    let (_m, _k, _n) = dimensions;

    #[cfg(feature = "cuda")]
    {
        // GPU-accelerated version
        execute_gemm_gpu(dimensions, seed)
    }
    #[cfg(not(feature = "cuda"))]
    {
        // CPU fallback
        execute_gemm_cpu(dimensions, seed)
    }
}

#[cfg(feature = "cuda")]
fn execute_gemm_gpu(dimensions: (usize, usize, usize), seed: u64) -> Result<(String, u64)> {
    let (m, k, n) = dimensions;
    
    // Use CUDA device for GPU acceleration
    let device = Device::new_cuda(0).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create CUDA device: {}", e))
    })?;

    // Generate random matrices using seed for reproducibility
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let normal = Normal::new(0.0, 1.0).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create normal distribution: {}", e))
    })?;

    // For H100 optimization: Use BF16 for tensor core acceleration
    // H100 has 4th gen tensor cores that excel at BF16
    let a_data: Vec<f32> = (0..m * k)
        .map(|_| normal.sample(&mut rng) as f32)
        .collect();
    
    let b_data: Vec<f32> = (0..k * n)
        .map(|_| normal.sample(&mut rng) as f32)
        .collect();

    // Create tensors on GPU with BF16 for maximum H100 tensor core utilization
    let a_tensor = Tensor::from_vec(a_data, (m, k), &device).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create tensor A: {}", e))
    })?.to_dtype(candle_core::DType::BF16).map_err(|e| {
        DemleError::ComputationError(format!("Failed to convert A to BF16: {}", e))
    })?;
    
    let b_tensor = Tensor::from_vec(b_data, (k, n), &device).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create tensor B: {}", e))
    })?.to_dtype(candle_core::DType::BF16).map_err(|e| {
        DemleError::ComputationError(format!("Failed to convert B to BF16: {}", e))
    })?;

    // Perform multiple GPU matrix multiplications for higher throughput
    // H100 can handle multiple concurrent operations
    let mut total_flops = 0u64;
    let mut all_results = Vec::new();
    
    // Execute many more operations to fully saturate H100 (increased from 4 to 16)
    for batch in 0..16 { // 16 concurrent batches for maximum H100 utilization
        let c_tensor = a_tensor.matmul(&b_tensor).map_err(|e| {
            DemleError::ComputationError(format!("GPU GEMM batch {} failed: {}", batch, e))
        })?;
        
        // Keep result on GPU until the end to minimize PCIe transfers
        all_results.push(c_tensor);
        total_flops += 2 * (m as u64) * (k as u64) * (n as u64);
    }

    // Single transfer back to CPU for hashing (minimize PCIe overhead)
    let final_result = &all_results[0]; // Use first result for hash
    let c_data: Vec<f32> = final_result.to_dtype(candle_core::DType::F32).map_err(|e| {
        DemleError::ComputationError(format!("Failed to convert result to F32: {}", e))
    })?.to_vec2().map_err(|e| {
        DemleError::ComputationError(format!("Failed to get result from GPU: {}", e))
    })?.into_iter().flatten().collect();

    // Hash the result (convert to FP8 for consistency)
    let fp8_data: Vec<FP8> = c_data.iter().map(|&f| FP8::from_f32(f)).collect();
    let result_bytes: Vec<u8> = fp8_data.iter().flat_map(|fp8| vec![fp8.to_bits()]).collect();
    let result_hash = Proof::hash_operation_result(&result_bytes);

    Ok((result_hash, total_flops))
}

fn execute_gemm_cpu(dimensions: (usize, usize, usize), seed: u64) -> Result<(String, u64)> {
    let (m, k, n) = dimensions;

    // Generate random matrices using seed for reproducibility
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let normal = Normal::new(0.0, 1.0).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create normal distribution: {}", e))
    })?;

    // Create matrices A(m×k) and B(k×n)
    let a_data: Vec<FP8> = (0..m * k)
        .map(|_| FP8::from_f32(normal.sample(&mut rng) as f32))
        .collect();

    let b_data: Vec<FP8> = (0..k * n)
        .map(|_| FP8::from_f32(normal.sample(&mut rng) as f32))
        .collect();

    // Perform GEMM in parallel
    let c_data: Vec<FP8> = (0..m * n)
        .into_par_iter()
        .map(|idx| {
            let i = idx / n;
            let j = idx % n;

            let mut sum = FP8::zero();
            for l in 0..k {
                let a_val = a_data[i * k + l];
                let b_val = b_data[l * n + j];
                sum = sum + (a_val * b_val);
            }
            sum
        })
        .collect();

    // Calculate FLOPS (2 * m * k * n for GEMM)
    let flops = 2 * (m as u64) * (k as u64) * (n as u64);

    // Hash the result
    let result_bytes: Vec<u8> = c_data.iter().flat_map(|fp8| vec![fp8.to_bits()]).collect();

    let result_hash = Proof::hash_operation_result(&result_bytes);

    Ok((result_hash, flops))
}

/// Optimized GEMM using blocked algorithm for better cache performance
pub fn execute_gemm_blocked(
    dimensions: (usize, usize, usize),
    seed: u64,
    block_size: usize,
) -> Result<(String, u64)> {
    let (m, k, n) = dimensions;

    // Generate matrices
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let normal = Normal::new(0.0, 1.0).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create normal distribution: {}", e))
    })?;

    let mut a = vec![FP8::zero(); m * k];
    let mut b = vec![FP8::zero(); k * n];
    let mut c = vec![FP8::zero(); m * n];

    // Initialize A and B
    for i in 0..m * k {
        a[i] = FP8::from_f32(normal.sample(&mut rng) as f32);
    }
    for i in 0..k * n {
        b[i] = FP8::from_f32(normal.sample(&mut rng) as f32);
    }

    // Blocked GEMM
    for ii in (0..m).step_by(block_size) {
        for jj in (0..n).step_by(block_size) {
            for kk in (0..k).step_by(block_size) {
                let i_end = (ii + block_size).min(m);
                let j_end = (jj + block_size).min(n);
                let k_end = (kk + block_size).min(k);

                for i in ii..i_end {
                    for j in jj..j_end {
                        let mut sum = c[i * n + j];
                        for l in kk..k_end {
                            sum = sum + (a[i * k + l] * b[l * n + j]);
                        }
                        c[i * n + j] = sum;
                    }
                }
            }
        }
    }

    let flops = 2 * (m as u64) * (k as u64) * (n as u64);
    let result_bytes: Vec<u8> = c.iter().flat_map(|fp8| vec![fp8.to_bits()]).collect();

    let result_hash = Proof::hash_operation_result(&result_bytes);

    Ok((result_hash, flops))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemm_execution() {
        let dimensions = (64, 64, 64);
        let seed = 42;

        let result = execute_gemm(dimensions, seed);
        assert!(result.is_ok());

        let (hash, flops) = result.unwrap();
        assert!(!hash.is_empty());
        assert_eq!(flops, 2 * 64 * 64 * 64);
    }

    #[test]
    fn test_gemm_deterministic() {
        let dimensions = (32, 32, 32);
        let seed = 123;

        let result1 = execute_gemm(dimensions, seed).unwrap();
        let result2 = execute_gemm(dimensions, seed).unwrap();

        // Same seed should produce same result
        assert_eq!(result1.0, result2.0);
        assert_eq!(result1.1, result2.1);
    }

    #[test]
    fn test_blocked_gemm() {
        let dimensions = (128, 128, 128);
        let seed = 456;
        let block_size = 32;

        let result = execute_gemm_blocked(dimensions, seed, block_size);
        assert!(result.is_ok());

        let (hash, flops) = result.unwrap();
        assert!(!hash.is_empty());
        assert_eq!(flops, 2 * 128 * 128 * 128);
    }
}
