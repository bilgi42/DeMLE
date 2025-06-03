use crate::fp8::FP8;
use crate::operations::generate_random_tensor;
use demle_core::{proof::Proof, Result};

/// Execute batch normalization operation
pub fn execute_batch_norm(
    shape: (usize, usize, usize, usize), // (batch, channels, height, width)
    epsilon: f32,
    seed: u64,
) -> Result<(String, u64)> {
    let (batch, channels, height, width) = shape;
    let total_size = batch * channels * height * width;

    // Generate random input data
    let input_data = generate_random_tensor(&[batch, channels, height, width], seed)?;

    // Generate random gamma and beta parameters
    let gamma = generate_random_tensor(&[channels], seed.wrapping_add(1))?;
    let beta = generate_random_tensor(&[channels], seed.wrapping_add(2))?;

    let mut output = vec![FP8::zero(); total_size];

    // Compute batch normalization for each channel
    for c in 0..channels {
        // Compute mean for this channel
        let mut sum = FP8::zero();
        let channel_size = batch * height * width;

        for b in 0..batch {
            for h in 0..height {
                for w in 0..width {
                    let idx = b * channels * height * width + c * height * width + h * width + w;
                    sum = sum + input_data[idx];
                }
            }
        }

        let mean = FP8::from_f32(sum.to_f32() / channel_size as f32);

        // Compute variance for this channel
        let mut var_sum = FP8::zero();
        for b in 0..batch {
            for h in 0..height {
                for w in 0..width {
                    let idx = b * channels * height * width + c * height * width + h * width + w;
                    let diff = input_data[idx] - mean;
                    var_sum = var_sum + (diff * diff);
                }
            }
        }

        let variance = FP8::from_f32(var_sum.to_f32() / channel_size as f32);
        let std_dev = FP8::from_f32((variance.to_f32() + epsilon).sqrt());

        // Normalize and apply scale/shift
        for b in 0..batch {
            for h in 0..height {
                for w in 0..width {
                    let idx = b * channels * height * width + c * height * width + h * width + w;

                    // Normalize
                    let normalized = (input_data[idx] - mean) / std_dev;

                    // Scale and shift
                    output[idx] = (normalized * gamma[c]) + beta[c];
                }
            }
        }
    }

    // Calculate FLOPS
    // Mean: N operations, Variance: 2N operations, Normalize: 3N operations per channel
    let flops_per_channel = 6 * (batch * height * width) as u64;
    let total_flops = (channels as u64) * flops_per_channel;

    // Hash the result
    let result_bytes: Vec<u8> = output.iter().flat_map(|fp8| vec![fp8.to_bits()]).collect();

    let result_hash = Proof::hash_operation_result(&result_bytes);

    Ok((result_hash, total_flops))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_norm_execution() {
        let shape = (4, 16, 8, 8); // Small batch
        let epsilon = 1e-5;
        let seed = 42;

        let result = execute_batch_norm(shape, epsilon, seed);
        assert!(result.is_ok());

        let (hash, flops) = result.unwrap();
        assert!(!hash.is_empty());
        assert!(flops > 0);
    }

    #[test]
    fn test_batch_norm_deterministic() {
        let shape = (2, 4, 4, 4);
        let epsilon = 1e-5;
        let seed = 123;

        let result1 = execute_batch_norm(shape, epsilon, seed).unwrap();
        let result2 = execute_batch_norm(shape, epsilon, seed).unwrap();

        assert_eq!(result1.0, result2.0);
        assert_eq!(result1.1, result2.1);
    }

    #[test]
    fn test_batch_norm_different_epsilon() {
        let shape = (1, 2, 3, 3);
        let seed = 456;

        let result1 = execute_batch_norm(shape, 1e-5, seed).unwrap();
        let result2 = execute_batch_norm(shape, 1e-4, seed).unwrap();

        // Different epsilon should give different results
        assert_ne!(result1.0, result2.0);
        assert_eq!(result1.1, result2.1); // But same FLOPS
    }
}
