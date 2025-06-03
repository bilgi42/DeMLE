use crate::fp8::FP8;
use crate::operations::generate_random_tensor;
use demle_core::{Result, DemleError, proof::Proof};

/// Execute 2D convolution operation
pub fn execute_conv2d(
    input_shape: (usize, usize, usize, usize), // (batch, channels, height, width)
    kernel_shape: (usize, usize, usize, usize), // (out_channels, in_channels, kh, kw)
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
                                        let input_idx = b * in_ch * ih * iw + 
                                                       ic * ih * iw + 
                                                       input_y * iw + 
                                                       input_x;
                                        
                                        let kernel_idx = oc * in_ch * kh * kw + 
                                                        ic * kh * kw + 
                                                        ky * kw + 
                                                        kx;
                                        
                                        sum = sum + (input_data[input_idx] * kernel_data[kernel_idx]);
                                    }
                                }
                            }
                        }
                    }
                    
                    let output_idx = b * out_ch * oh * ow + 
                                    oc * oh * ow + 
                                    y * ow + 
                                    x;
                    output[output_idx] = sum;
                }
            }
        }
    }
    
    // Calculate FLOPS
    let flops = 2 * (batch as u64) * (out_ch as u64) * (in_ch as u64) * 
                (kh as u64) * (kw as u64) * (oh as u64) * (ow as u64);
    
    // Hash the result
    let result_bytes: Vec<u8> = output.iter()
        .flat_map(|fp8| vec![fp8.to_bits()])
        .collect();
    
    let result_hash = Proof::hash_operation_result(&result_bytes);
    
    Ok((result_hash, flops))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv2d_execution() {
        let input_shape = (1, 3, 32, 32);   // Small image
        let kernel_shape = (16, 3, 3, 3);   // 16 filters, 3x3 kernels
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