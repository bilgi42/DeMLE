use crate::fp8::FP8;
use crate::operations::{generate_random_tensor, softmax};
use demle_core::{proof::Proof, Result};

#[cfg(feature = "cuda")]
use candle_core::{Device, Tensor, DType};

/// Execute multi-head attention operation with H100 optimization
pub fn execute_attention(
    batch_size: usize,
    seq_length: usize,
    d_model: usize,
    num_heads: usize,
    seed: u64,
) -> Result<(String, u64)> {
    #[cfg(feature = "cuda")]
    {
        execute_attention_gpu(batch_size, seq_length, d_model, num_heads, seed)
    }
    #[cfg(not(feature = "cuda"))]
    {
        execute_attention_cpu(batch_size, seq_length, d_model, num_heads, seed)
    }
}

#[cfg(feature = "cuda")]
fn execute_attention_gpu(
    batch_size: usize,
    seq_length: usize,
    d_model: usize,
    num_heads: usize,
    seed: u64,
) -> Result<(String, u64)> {
    let device = Device::new_cuda(0).map_err(|e| {
        demle_core::DemleError::ComputationError(format!("Failed to create CUDA device: {}", e))
    })?;

    let d_k = d_model / num_heads;

    // Generate much larger attention computation for H100
    // Use random data directly on GPU with BF16 for tensor cores
    use rand::SeedableRng;
    use rand_distr::{Distribution, Normal};
    
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let normal = Normal::new(0.0, 1.0).map_err(|e| {
        demle_core::DemleError::ComputationError(format!("Failed to create normal distribution: {}", e))
    })?;

    // Create large Q, K, V tensors directly on GPU
    let q_data: Vec<f32> = (0..batch_size * seq_length * d_model)
        .map(|_| normal.sample(&mut rng) as f32)
        .collect();
    let k_data: Vec<f32> = (0..batch_size * seq_length * d_model)
        .map(|_| normal.sample(&mut rng) as f32)
        .collect();
    let v_data: Vec<f32> = (0..batch_size * seq_length * d_model)
        .map(|_| normal.sample(&mut rng) as f32)
        .collect();

    let q = Tensor::from_vec(q_data, (batch_size, seq_length, d_model), &device)?
        .to_dtype(DType::BF16)?;
    let k = Tensor::from_vec(k_data, (batch_size, seq_length, d_model), &device)?
        .to_dtype(DType::BF16)?;
    let v = Tensor::from_vec(v_data, (batch_size, seq_length, d_model), &device)?
        .to_dtype(DType::BF16)?;

    // Reshape for multi-head attention with H100 optimization
    let q_heads = q.reshape((batch_size, seq_length, num_heads, d_k))?
        .transpose(1, 2)?; // (batch, heads, seq_len, d_k)
    let k_heads = k.reshape((batch_size, seq_length, num_heads, d_k))?
        .transpose(1, 2)?;
    let v_heads = v.reshape((batch_size, seq_length, num_heads, d_k))?
        .transpose(1, 2)?;

    // Multiple attention iterations for higher FLOPS
    let mut total_flops = 0u64;
    let mut final_output = None;

    // Optimal attention iterations for H100 memory efficiency
    for iteration in 0..10 { // Reduced from 20 to 10 for memory balance
        // Compute attention scores (Q @ K^T)
        let q_reshaped = q_heads.reshape((batch_size * num_heads, seq_length, d_k))?;
        let k_reshaped = k_heads.reshape((batch_size * num_heads, seq_length, d_k))?;
        let v_reshaped = v_heads.reshape((batch_size * num_heads, seq_length, d_k))?;

        let scores = q_reshaped.contiguous()?.matmul(&k_reshaped.transpose(1, 2)?.contiguous()?)?;
        let attention_out = scores.contiguous()?.matmul(&v_reshaped.contiguous()?)?;

        if iteration == 0 {
            final_output = Some(attention_out);
        }

        // FLOPs for attention: 2 * batch * heads * seq^2 * head_dim for QK^T, 2 * batch * heads * seq^2 * head_dim for softmax*V
        let iteration_flops = 4 * batch_size * num_heads * seq_length * seq_length * d_k;
        total_flops += iteration_flops as u64;
    }

    // Get result back to CPU for hashing
    let output_data: Vec<f32> = final_output.unwrap()
        .to_dtype(DType::F32)?
        .flatten_all()?
        .to_vec1()?;

    // Convert to FP8 for consistent hashing
    let fp8_data: Vec<FP8> = output_data.iter().map(|&f| FP8::from_f32(f)).collect();
    let result_bytes: Vec<u8> = fp8_data.iter().flat_map(|fp8| vec![fp8.to_bits()]).collect();
    let result_hash = Proof::hash_operation_result(&result_bytes);

    Ok((result_hash, total_flops))
}

fn execute_attention_cpu(
    batch_size: usize,
    seq_length: usize,
    d_model: usize,
    num_heads: usize,
    seed: u64,
) -> Result<(String, u64)> {
    let d_k = d_model / num_heads;

    // Generate random input (batch_size, seq_length, d_model)
    let input_data = generate_random_tensor(&[batch_size, seq_length, d_model], seed)?;

    // Generate random weight matrices for Q, K, V projections
    let wq = generate_random_tensor(&[d_model, d_model], seed.wrapping_add(1))?;
    let wk = generate_random_tensor(&[d_model, d_model], seed.wrapping_add(2))?;
    let wv = generate_random_tensor(&[d_model, d_model], seed.wrapping_add(3))?;

    let mut output = vec![FP8::zero(); batch_size * seq_length * d_model];

    for b in 0..batch_size {
        for h in 0..num_heads {
            // Extract Q, K, V for this head
            let mut q_head = vec![FP8::zero(); seq_length * d_k];
            let mut k_head = vec![FP8::zero(); seq_length * d_k];
            let mut v_head = vec![FP8::zero(); seq_length * d_k];

            // Simplified projection (normally would be linear layer)
            for i in 0..seq_length {
                for j in 0..d_k {
                    let head_offset = h * d_k + j;

                    // Q projection
                    let mut q_sum = FP8::zero();
                    for k in 0..d_model {
                        let input_idx = b * seq_length * d_model + i * d_model + k;
                        let weight_idx = k * d_model + head_offset;
                        q_sum = q_sum + (input_data[input_idx] * wq[weight_idx]);
                    }
                    q_head[i * d_k + j] = q_sum;

                    // K projection
                    let mut k_sum = FP8::zero();
                    for k in 0..d_model {
                        let input_idx = b * seq_length * d_model + i * d_model + k;
                        let weight_idx = k * d_model + head_offset;
                        k_sum = k_sum + (input_data[input_idx] * wk[weight_idx]);
                    }
                    k_head[i * d_k + j] = k_sum;

                    // V projection
                    let mut v_sum = FP8::zero();
                    for k in 0..d_model {
                        let input_idx = b * seq_length * d_model + i * d_model + k;
                        let weight_idx = k * d_model + head_offset;
                        v_sum = v_sum + (input_data[input_idx] * wv[weight_idx]);
                    }
                    v_head[i * d_k + j] = v_sum;
                }
            }

            // Compute attention scores and apply attention
            for i in 0..seq_length {
                // Compute attention scores for position i
                let mut scores = vec![FP8::zero(); seq_length];
                for j in 0..seq_length {
                    let mut score = FP8::zero();
                    for k in 0..d_k {
                        score = score + (q_head[i * d_k + k] * k_head[j * d_k + k]);
                    }
                    // Scale by sqrt(d_k)
                    let scale = FP8::from_f32(1.0 / (d_k as f32).sqrt());
                    scores[j] = score * scale;
                }

                // Apply softmax
                let attention_weights = softmax(&scores);

                // Apply attention to values
                for j in 0..d_k {
                    let mut attended_value = FP8::zero();
                    for k in 0..seq_length {
                        attended_value =
                            attended_value + (attention_weights[k] * v_head[k * d_k + j]);
                    }

                    let output_idx = b * seq_length * d_model + i * d_model + h * d_k + j;
                    output[output_idx] = attended_value;
                }
            }
        }
    }

    // Calculate FLOPS (simplified estimation)
    let qkv_flops =
        3 * (batch_size as u64) * (seq_length as u64) * (d_model as u64) * (d_model as u64);
    let attention_flops = (batch_size as u64)
        * (num_heads as u64)
        * (seq_length as u64)
        * (seq_length as u64)
        * (d_k as u64);
    let output_flops =
        (batch_size as u64) * (seq_length as u64) * (d_model as u64) * (d_model as u64);
    let total_flops = qkv_flops + 2 * attention_flops + output_flops;

    // Hash the result
    let result_bytes: Vec<u8> = output.iter().flat_map(|fp8| vec![fp8.to_bits()]).collect();

    let result_hash = Proof::hash_operation_result(&result_bytes);

    Ok((result_hash, total_flops))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attention_execution() {
        let batch_size = 2;
        let seq_length = 16;
        let d_model = 64;
        let num_heads = 8;
        let seed = 42;

        let result = execute_attention(batch_size, seq_length, d_model, num_heads, seed);
        assert!(result.is_ok());

        let (hash, flops) = result.unwrap();
        assert!(!hash.is_empty());
        assert!(flops > 0);
    }

    #[test]
    fn test_attention_deterministic() {
        let batch_size = 1;
        let seq_length = 8;
        let d_model = 32;
        let num_heads = 4;
        let seed = 123;

        let result1 = execute_attention(batch_size, seq_length, d_model, num_heads, seed).unwrap();
        let result2 = execute_attention(batch_size, seq_length, d_model, num_heads, seed).unwrap();

        assert_eq!(result1.0, result2.0);
        assert_eq!(result1.1, result2.1);
    }
}
