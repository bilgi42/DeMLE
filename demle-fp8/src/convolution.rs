use crate::fp8::FP8;
use crate::operations::generate_random_tensor;
use demle_core::{proof::Proof, DemleError, Result};

#[cfg(feature = "cuda")]
use candle_core::{Device, Tensor};
#[cfg(feature = "cuda")]
use rand::SeedableRng;
#[cfg(feature = "cuda")]
use rand_distr::{Distribution, Normal};

/// Execute 2D convolution operation
/// Uses GPU acceleration when available
pub fn execute_conv2d(
    input_shape: (usize, usize, usize, usize), // (batch, channels, height, width)
    kernel_shape: (usize, usize, usize, usize), // (out_channels, in_channels, kh, kw)
    stride: (usize, usize),
    padding: (usize, usize),
    seed: u64,
) -> Result<(String, u64)> {
    #[cfg(feature = "cuda")]
    {
        // GPU-accelerated version
        execute_conv2d_gpu(input_shape, kernel_shape, stride, padding, seed)
    }
    #[cfg(not(feature = "cuda"))]
    {
        // CPU fallback
        execute_conv2d_cpu(input_shape, kernel_shape, stride, padding, seed)
    }
}

#[cfg(feature = "cuda")]
fn execute_conv2d_gpu(
    input_shape: (usize, usize, usize, usize),
    kernel_shape: (usize, usize, usize, usize),
    stride: (usize, usize),
    padding: (usize, usize),
    seed: u64,
) -> Result<(String, u64)> {
    let (batch, in_ch, ih, iw) = input_shape;
    let (out_ch, _, kh, kw) = kernel_shape;
    let (sh, sw) = stride;
    let (ph, pw) = padding;

    // Use CUDA device for GPU acceleration
    let device = Device::new_cuda(0).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create CUDA device: {}", e))
    })?;

    // Generate random input and kernel data
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let normal = Normal::new(0.0, 1.0).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create normal distribution: {}", e))
    })?;

    let input_data: Vec<f32> = (0..(batch * in_ch * ih * iw))
        .map(|_| normal.sample(&mut rng) as f32)
        .collect();

    let kernel_data: Vec<f32> = (0..(out_ch * in_ch * kh * kw))
        .map(|_| normal.sample(&mut rng) as f32)
        .collect();

    // Create tensors on GPU with BF16 for H100 tensor core acceleration
    let input_tensor = Tensor::from_vec(input_data, (batch, in_ch, ih, iw), &device).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create input tensor: {}", e))
    })?.to_dtype(candle_core::DType::BF16).map_err(|e| {
        DemleError::ComputationError(format!("Failed to convert input to BF16: {}", e))
    })?;

    let kernel_tensor = Tensor::from_vec(kernel_data, (out_ch, in_ch, kh, kw), &device).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create kernel tensor: {}", e))
    })?.to_dtype(candle_core::DType::BF16).map_err(|e| {
        DemleError::ComputationError(format!("Failed to convert kernel to BF16: {}", e))
    })?;

    // Perform optimal convolution batches for H100 memory management
    let mut total_flops = 0u64;
    let mut final_output = None;

    // Execute optimal convolution batches for H100 memory management
    for conv_batch in 0..6 { // Reduced from 12 to 6 for memory efficiency
        let output_tensor = input_tensor.conv2d(&kernel_tensor, ph, pw, sh, sw).map_err(|e| {
            DemleError::ComputationError(format!("GPU Conv2D batch {} failed: {}", conv_batch, e))
        })?;

        if conv_batch == 0 {
            final_output = Some(output_tensor);
        }

        // Calculate FLOPS for this convolution
        let oh = (ih + 2 * ph - kh) / sh + 1;
        let ow = (iw + 2 * pw - kw) / sw + 1;
        let batch_flops = 2 * (batch as u64) * (out_ch as u64) * (in_ch as u64) * 
                         (kh as u64) * (kw as u64) * (oh as u64) * (ow as u64);
        total_flops += batch_flops;
    }

    // Get result back to CPU for hashing (single transfer to minimize overhead)
    let output_data: Vec<f32> = final_output.unwrap()
        .to_dtype(candle_core::DType::F32).map_err(|e| {
            DemleError::ComputationError(format!("Failed to convert output to F32: {}", e))
        })?.flatten_all().map_err(|e| {
            DemleError::ComputationError(format!("Failed to flatten output: {}", e))
        })?.to_vec1().map_err(|e| {
            DemleError::ComputationError(format!("Failed to get result from GPU: {}", e))
        })?;

    // Hash the result (convert to FP8 for consistency)
    let fp8_data: Vec<FP8> = output_data.iter().map(|&f| FP8::from_f32(f)).collect();
    let result_bytes: Vec<u8> = fp8_data.iter().flat_map(|fp8| vec![fp8.to_bits()]).collect();
    let result_hash = Proof::hash_operation_result(&result_bytes);

    Ok((result_hash, total_flops))
}

fn execute_conv2d_cpu(
    input_shape: (usize, usize, usize, usize),
    kernel_shape: (usize, usize, usize, usize),
    stride: (usize, usize),
    padding: (usize, usize),
    seed: u64,
) -> Result<(String, u64)> {
    let (batch, in_ch, ih, iw) = input_shape;
    let (out_ch, _, kh, kw) = kernel_shape;
    let (sh, sw) = stride;
    let (ph, pw) = padding;

    // Calculate output dimensions
    let oh = (ih + 2 * ph - kh) / sh + 1;
    let ow = (iw + 2 * pw - kw) / sw + 1;

    // Generate random input and kernel tensors
    let input_data = generate_random_tensor(&[batch, in_ch, ih, iw], seed)?;
    let kernel_data = generate_random_tensor(&[out_ch, in_ch, kh, kw], seed.wrapping_add(1))?;

    // Initialize output
    let mut output = vec![FP8::zero(); batch * out_ch * oh * ow];

    // Perform convolution
    for b in 0..batch {
        for oc in 0..out_ch {
            for y in 0..oh {
                for x in 0..ow {
                    let mut sum = FP8::zero();

                    for ic in 0..in_ch {
                        for ky in 0..kh {
                            for kx in 0..kw {
                                let input_y = y * sh + ky;
                                let input_x = x * sw + kx;

                                // Handle padding
                                if input_y >= ph && input_x >= pw {
                                    let input_y = input_y - ph;
                                    let input_x = input_x - pw;

                                    if input_y < ih && input_x < iw {
                                        let input_idx = b * in_ch * ih * iw
                                            + ic * ih * iw
                                            + input_y * iw
                                            + input_x;

                                        let kernel_idx =
                                            oc * in_ch * kh * kw + ic * kh * kw + ky * kw + kx;

                                        sum =
                                            sum + (input_data[input_idx] * kernel_data[kernel_idx]);
                                    }
                                }
                            }
                        }
                    }

                    let output_idx = b * out_ch * oh * ow + oc * oh * ow + y * ow + x;
                    output[output_idx] = sum;
                }
            }
        }
    }

    // Calculate FLOPS
    let flops = 2
        * (batch as u64)
        * (out_ch as u64)
        * (in_ch as u64)
        * (kh as u64)
        * (kw as u64)
        * (oh as u64)
        * (ow as u64);

    // Hash the result
    let result_bytes: Vec<u8> = output.iter().flat_map(|fp8| vec![fp8.to_bits()]).collect();

    let result_hash = Proof::hash_operation_result(&result_bytes);

    Ok((result_hash, flops))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv2d_execution() {
        let input_shape = (1, 3, 32, 32); // Small image
        let kernel_shape = (16, 3, 3, 3); // 16 filters, 3x3 kernels
        let stride = (1, 1);
        let padding = (0, 0);
        let seed = 42;

        let result = execute_conv2d(input_shape, kernel_shape, stride, padding, seed);
        assert!(result.is_ok());

        let (hash, flops) = result.unwrap();
        assert!(!hash.is_empty());
        assert!(flops > 0);
    }

    #[test]
    fn test_conv2d_deterministic() {
        let input_shape = (1, 1, 8, 8);
        let kernel_shape = (1, 1, 3, 3);
        let stride = (1, 1);
        let padding = (0, 0);
        let seed = 123;

        let result1 = execute_conv2d(input_shape, kernel_shape, stride, padding, seed).unwrap();
        let result2 = execute_conv2d(input_shape, kernel_shape, stride, padding, seed).unwrap();

        assert_eq!(result1.0, result2.0);
        assert_eq!(result1.1, result2.1);
    }
}
