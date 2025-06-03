/// Calculate new difficulty based on block times
pub fn adjust_difficulty(
    current_difficulty: u64,
    actual_time: u64,
    target_time: u64,
    max_adjustment: f64,
) -> u64 {
    let time_ratio = actual_time as f64 / target_time as f64;

    // Clamp adjustment to prevent extreme changes
    let adjustment = time_ratio.max(1.0 / max_adjustment).min(max_adjustment);

    // Apply adjustment
    let new_difficulty = (current_difficulty as f64 / adjustment) as u64;

    // Ensure minimum difficulty
    new_difficulty.max(1000)
}

/// Calculate target difficulty for given teraflops
pub fn teraflops_to_difficulty(teraflops: f64) -> u64 {
    // Convert teraflops to difficulty units
    // Higher teraflops = higher difficulty
    (teraflops * 1e6) as u64
}

/// Calculate required teraflops for difficulty
pub fn difficulty_to_teraflops(difficulty: u64) -> f64 {
    difficulty as f64 / 1e6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difficulty_adjustment() {
        let current = 1000000;
        let target = 15; // 15 seconds

        // Blocks too fast - increase difficulty
        let new_diff = adjust_difficulty(current, 10, target, 2.0);
        assert!(new_diff > current);

        // Blocks too slow - decrease difficulty
        let new_diff = adjust_difficulty(current, 20, target, 2.0);
        assert!(new_diff < current);
    }

    #[test]
    fn test_teraflops_conversion() {
        let teraflops = 1.5;
        let difficulty = teraflops_to_difficulty(teraflops);
        let converted_back = difficulty_to_teraflops(difficulty);

        assert!((converted_back - teraflops).abs() < 0.001);
    }
}
