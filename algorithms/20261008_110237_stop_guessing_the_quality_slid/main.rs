mod types;
mod engine;

use types::{QualitySearchResult, SearchConfig};
use engine::{binary_search_quality, find_min_quality_for_size, find_max_quality_for_size, simulate_jpeg_size};

fn main() {
    // Test 1: Basic binary search for target size
    let config = SearchConfig::new(1, 100, 5000, 100);
    let result = binary_search_quality(&config);
    println!("Test 1: Target 5000 bytes, tolerance 100");
    println!("  Quality: {}, Size: {}, Success: {}", result.quality, result.estimated_size, result.success);
    assert!(result.success, "Should find quality within tolerance");
    assert!(result.estimated_size.abs_diff(5000) <= 100, "Size should be within tolerance");

    // Test 2: Target size that is exactly achievable
    // simulate_jpeg_size(10) = 1000 + 100*10 + 10*50 = 1000 + 1000 + 500 = 2500
    let config2 = SearchConfig::new(1, 100, 2500, 0);
    let result2 = binary_search_quality(&config2);
    println!("Test 2: Target 2500 bytes (exact), tolerance 0");
    println!("  Quality: {}, Size: {}, Success: {}", result2.quality, result2.estimated_size, result2.success);
    assert_eq!(result2.quality, 10, "Should find exact quality 10");
    assert_eq!(result2.estimated_size, 2500, "Size should be exactly 2500");
    assert!(result2.success, "Exact match should succeed");

    // Test 3: Target size below minimum possible
    // simulate_jpeg_size(1) = 1000 + 10 + 50 = 1060
    let config3 = SearchConfig::new(1, 100, 500, 100);
    let result3 = binary_search_quality(&config3);
    println!("Test 3: Target 500 bytes (below min), tolerance 100");
    println!("  Quality: {}, Size: {}, Success: {}", result3.quality, result3.estimated_size, result3.success);
    assert_eq!(result3.quality, 1, "Should return minimum quality");
    assert_eq!(result3.estimated_size, 1060, "Size should be minimum possible");
    assert!(!result3.success, "Should fail because target is too low");

    // Test 4: Target size above maximum possible
    // simulate_jpeg_size(100) = 1000 + 10000*10 + 100*50 = 1000 + 100000 + 5000 = 106000
    let config4 = SearchConfig::new(1, 100, 200000, 100);
    let result4 = binary_search_quality(&config4);
    println!("Test 4: Target 200000 bytes (above max), tolerance 100");
    println!("  Quality: {}, Size: {}, Success: {}", result4.quality, result4.estimated_size, result4.success);
    assert_eq!(result4.quality, 100, "Should return maximum quality");
    assert_eq!(result4.estimated_size, 106000, "Size should be maximum possible");
    assert!(!result4.success, "Should fail because target is too high");

    // Test 5: find_min_quality_for_size
    // Find min quality where size >= 2500
    let min_q = find_min_quality_for_size(2500, 1, 100);
    println!("Test 5: Min quality for size >= 2500: {:?}", min_q);
    assert_eq!(min_q, Some(10), "Min quality for 2500 should be 10");

    // Test 6: find_max_quality_for_size
    // Find max quality where size <= 2500
    let max_q = find_max_quality_for_size(2500, 1, 100);
    println!("Test 6: Max quality for size <= 2500: {:?}", max_q);
    assert_eq!(max_q, Some(10), "Max quality for 2500 should be 10");

    // Test 7: find_min_quality_for_size with target below minimum
    let min_q_low = find_min_quality_for_size(500, 1, 100);
    println!("Test 7: Min quality for size >= 500: {:?}", min_q_low);
    assert_eq!(min_q_low, Some(1), "Min quality for 500 should be 1 (smallest)");

    // Test 8: find_max_quality_for_size with target above maximum
    let max_q_high = find_max_quality_for_size(200000, 1, 100);
    println!("Test 8: Max quality for size <= 200000: {:?}", max_q_high);
    assert_eq!(max_q_high, Some(100), "Max quality for 200000 should be 100 (largest)");

    // Test 9: Verify monotonicity of size function
    let mut prev_size = simulate_jpeg_size(0);
    for q in 1..=100u8 {
        let size = simulate_jpeg_size(q);
        assert!(size > prev_size, "Size function must be strictly increasing");
        prev_size = size;
    }
    println!("Test 9: Monotonicity verified for all quality levels 0-100");

    // Test 10: Edge case - single quality range
    let config10 = SearchConfig::new(50, 50, 2500, 100);
    let result10 = binary_search_quality(&config10);
    println!("Test 10: Single quality 50, target 2500");
    println!("  Quality: {}, Size: {}, Success: {}", result10.quality, result10.estimated_size, result10.success);
    assert_eq!(result10.quality, 50, "Should return quality 50");
    // simulate_jpeg_size(50) = 1000 + 2500*10 + 50*50 = 1000 + 25000 + 2500 = 28500
    assert_eq!(result10.estimated_size, 28500, "Size for quality 50 should be 28500");

    // Test 11: Verify binary search correctness against brute force
    for target in [1500, 3000, 5000, 10000, 25000, 50000, 75000, 100000] {
        let config = SearchConfig::new(1, 100, target, 0);
        let bs_result = binary_search_quality(&config);

        // Brute force: find quality with minimum diff
        let mut best_q = 1u8;
        let mut best_diff = u64::MAX;
        for q in 1..=100u8 {
            let size = simulate_jpeg_size(q);
            let diff = size.abs_diff(target);
            if diff < best_diff {
                best_diff = diff;
                best_q = q;
            }
        }

        assert_eq!(bs_result.quality, best_q, "Binary search should match brute force for target {}", target);
        assert_eq!(bs_result.estimated_size, simulate_jpeg_size(best_q), "Size should match for target {}", target);
    }
    println!("Test 11: Binary search matches brute force for 8 target sizes");

    // Test 12: Tolerance boundary
    // simulate_jpeg_size(20) = 1000 + 400*10 + 20*50 = 1000 + 4000 + 1000 = 6000
    // simulate_jpeg_size(21) = 1000 + 441*10 + 21*50 = 1000 + 4410 + 1050 = 6460
    // Target 6200, tolerance 200: diff from 6000 is 200 (within), diff from 6460 is 260 (outside)
    let config12 = SearchConfig::new(1, 100, 6200, 200);
    let result12 = binary_search_quality(&config12);
    println!("Test 12: Target 6200, tolerance 200");
    println!("  Quality: {}, Size: {}, Success: {}", result12.quality, result12.estimated_size, result12.success);
    assert!(result12.success, "Should succeed with tolerance 200");
    assert!(result12.estimated_size.abs_diff(6200) <= 200, "Size within tolerance");

    println!("\nAll tests passed successfully!");
}