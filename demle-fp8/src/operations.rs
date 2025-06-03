use crate::fp8::FP8;
use demle_core::{DemleError, Result};
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};

/// Generate random FP8 tensor with given shape and seed
pub fn generate_random_tensor(shape: &[usize], seed: u64) -> Result<Vec<FP8>> {
    let total_size = shape.iter().product();
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let normal = Normal::new(0.0, 1.0).map_err(|e| {
        DemleError::ComputationError(format!("Failed to create normal distribution: {}", e))
    })?;

    let data: Vec<FP8> = (0..total_size)
        .map(|_| FP8::from_f32(normal.sample(&mut rng) as f32))
        .collect();

    Ok(data)
}

/// Apply activation function to tensor
pub fn apply_activation(data: &[FP8], activation: ActivationType) -> Vec<FP8> {
    data.iter()
        .map(|&x| match activation {
            ActivationType::ReLU => relu(x),
            ActivationType::GELU => gelu(x),
            ActivationType::Swish => swish(x),
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub enum ActivationType {
    ReLU,
    GELU,
    Swish,
}

/// ReLU activation function
fn relu(x: FP8) -> FP8 {
    let val = x.to_f32();
    FP8::from_f32(val.max(0.0))
}

/// GELU activation function (approximation)
fn gelu(x: FP8) -> FP8 {
    let val = x.to_f32();
    let result = 0.5 * val * (1.0 + (0.7978845608 * (val + 0.044715 * val.powi(3))).tanh());
    FP8::from_f32(result)
}

/// Swish activation function
fn swish(x: FP8) -> FP8 {
    let val = x.to_f32();
    let sigmoid = 1.0 / (1.0 + (-val).exp());
    FP8::from_f32(val * sigmoid)
}

/// Softmax function for attention computation
pub fn softmax(input: &[FP8]) -> Vec<FP8> {
    let max_val = input
        .iter()
        .map(|x| x.to_f32())
        .fold(f32::NEG_INFINITY, f32::max);

    let exp_values: Vec<f32> = input.iter().map(|x| (x.to_f32() - max_val).exp()).collect();

    let sum: f32 = exp_values.iter().sum();

    exp_values.iter().map(|&x| FP8::from_f32(x / sum)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_tensor() {
        let shape = [2, 3, 4];
        let seed = 42;

        let tensor = generate_random_tensor(&shape, seed).unwrap();
        assert_eq!(tensor.len(), 24);

        // Same seed should generate same tensor
        let tensor2 = generate_random_tensor(&shape, seed).unwrap();
        assert_eq!(tensor, tensor2);
    }

    #[test]
    fn test_activations() {
        let input = vec![
            FP8::from_f32(-1.0),
            FP8::from_f32(0.0),
            FP8::from_f32(1.0),
            FP8::from_f32(2.0),
        ];

        let relu_output = apply_activation(&input, ActivationType::ReLU);
        assert_eq!(relu_output[0].to_f32(), 0.0); // ReLU(-1) = 0
        assert!(relu_output[3].to_f32() > 0.0); // ReLU(2) > 0

        let gelu_output = apply_activation(&input, ActivationType::GELU);
        assert!(gelu_output.len() == input.len());

        let swish_output = apply_activation(&input, ActivationType::Swish);
        assert!(swish_output.len() == input.len());
    }

    #[test]
    fn test_softmax() {
        let input = vec![FP8::from_f32(1.0), FP8::from_f32(2.0), FP8::from_f32(3.0)];

        let output = softmax(&input);

        // Softmax should sum to 1
        let sum: f32 = output.iter().map(|x| x.to_f32()).sum();
        assert!((sum - 1.0).abs() < 0.01);

        // Larger inputs should have larger softmax values
        assert!(output[2].to_f32() > output[1].to_f32());
        assert!(output[1].to_f32() > output[0].to_f32());
    }
}
