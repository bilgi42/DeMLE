use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Sub};

/// FP8 data type using E4M3 format (1 sign + 4 exponent + 3 mantissa bits)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FP8 {
    bits: u8,
}

impl FP8 {
    /// Create FP8 from raw bits
    pub fn from_bits(bits: u8) -> Self {
        Self { bits }
    }

    /// Get raw bits
    pub fn to_bits(self) -> u8 {
        self.bits
    }

    /// Create FP8 zero
    pub fn zero() -> Self {
        Self { bits: 0 }
    }

    /// Create FP8 one
    pub fn one() -> Self {
        Self::from_f32(1.0)
    }

    /// Convert from f32 to FP8 E4M3 format
    pub fn from_f32(value: f32) -> Self {
        if value == 0.0 {
            return Self::zero();
        }

        let bits = value.to_bits();
        let sign = (bits >> 31) & 1;
        let exp = ((bits >> 23) & 0xFF) as i32;
        let mantissa = (bits >> 20) & 0x7; // Take top 3 bits of mantissa

        // Adjust exponent for FP8 bias (bias = 7 for 4-bit exponent)
        let fp8_exp = (exp - 127 + 7).clamp(0, 15) as u8;

        let fp8_bits = ((sign as u8) << 7) | (fp8_exp << 3) | (mantissa as u8);
        Self { bits: fp8_bits }
    }

    /// Convert from FP8 to f32
    pub fn to_f32(self) -> f32 {
        if self.bits == 0 {
            return 0.0;
        }

        let sign = (self.bits >> 7) & 1;
        let exp = (self.bits >> 3) & 0xF;
        let mantissa = self.bits & 0x7;

        // Convert back to f32 format
        let f32_exp = if exp == 0 {
            0 // Handle denormals as zero for simplicity
        } else {
            ((exp as i32) - 7 + 127) as u32
        };

        let f32_mantissa = (mantissa as u32) << 20;
        let f32_bits = ((sign as u32) << 31) | (f32_exp << 23) | f32_mantissa;

        f32::from_bits(f32_bits)
    }
}

impl From<f32> for FP8 {
    fn from(value: f32) -> Self {
        Self::from_f32(value)
    }
}

impl From<FP8> for f32 {
    fn from(value: FP8) -> Self {
        value.to_f32()
    }
}

impl Add for FP8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::from_f32(self.to_f32() + rhs.to_f32())
    }
}

impl Mul for FP8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::from_f32(self.to_f32() * rhs.to_f32())
    }
}

impl Sub for FP8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::from_f32(self.to_f32() - rhs.to_f32())
    }
}

impl Div for FP8 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::from_f32(self.to_f32() / rhs.to_f32())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fp8_conversion() {
        let values = [0.0, 1.0, -1.0, 2.0, 0.5, -0.5];

        for &val in &values {
            let fp8 = FP8::from_f32(val);
            let converted = fp8.to_f32();

            // Allow some precision loss
            assert!(
                (converted - val).abs() < 0.1,
                "Failed for {}: got {}",
                val,
                converted
            );
        }
    }

    #[test]
    fn test_fp8_arithmetic() {
        let a = FP8::from_f32(2.0);
        let b = FP8::from_f32(3.0);

        let sum = (a + b).to_f32();
        assert!((sum - 5.0).abs() < 0.1);

        let product = (a * b).to_f32();
        assert!((product - 6.0).abs() < 0.1);
    }
}
