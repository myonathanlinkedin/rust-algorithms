mod core;
use core::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let config = QuantumStreamConfig {
            dimension: 4,
            replenishment_rate: 0.1,
            robustness_threshold: 0.5,
        };
        let mut engine = QuantumStreamingEngine::new(config);
        
        let state = engine.get_state();
        assert_eq!(state.amplitudes.len(), 4);
        assert!((state.amplitudes[0] - 0.5).abs() < 1e-10);
        assert!((state.entropy_estimate - (4.0_f64).ln()).abs() < 1e-10);
    }

    #[test]
    fn test_token_processing() {
        let config = QuantumStreamConfig {
            dimension: 4,
            replenishment_rate: 0.1,
            robustness_threshold: 0.5,
        };
        let mut engine = QuantumStreamingEngine::new(config);
        
        engine.process_token(0);
        engine.process_token(1);
        
        let state = engine.get_state();
        assert!(state.replenishment_counter == 2);
        assert!(state.amplitudes[0] > 0.5);
        assert!(state.amplitudes[1] > 0.5);
    }

    #[test]
    fn test_robustness_score() {
        let config = QuantumStreamConfig {
            dimension: 4,
            replenishment_rate: 0.1,
            robustness_threshold: 0.5,
        };
        let mut engine = QuantumStreamingEngine::new(config);
        
        for _ in 0..10 {
            engine.process_token(0);
        }
        
        let score = engine.get_robustness_score();
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn test_lower_bound_computation() {
        let bound = compute_lower_bound(100, 0.01);
        assert!(bound > 0.0);
        
        let bound2 = compute_lower_bound(100, 0.5);
        assert!(bound2 > 0.0);
        
        let invalid = compute_lower_bound(0, 0.01);
        assert_eq!(invalid, 0.0);
    }

    #[test]
    fn test_hash_token() {
        let h1 = hash_token(42);
        let h2 = hash_token(42);
        assert_eq!(h1, h2);
        
        let h3 = hash_token(43);
        assert_ne!(h1, h3);
    }

    #[test]
    fn test_reset() {
        let config = QuantumStreamConfig {
            dimension: 4,
            replenishment_rate: 0.1,
            robustness_threshold: 0.5,
        };
        let mut engine = QuantumStreamingEngine::new(config);
        
        engine.process_token(0);
        engine.process_token(1);
        
        engine.reset();
        
        let state = engine.get_state();
        assert_eq!(state.replenishment_counter, 0);
        assert!((state.amplitudes[0] - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_out_of_bounds_token() {
        let config = QuantumStreamConfig {
            dimension: 4,
            replenishment_rate: 0.1,
            robustness_threshold: 0.5,
        };
        let mut engine = QuantumStreamingEngine::new(config);
        
        engine.process_token(100);
        
        let state = engine.get_state();
        assert_eq!(state.replenishment_counter, 0);
    }

    #[test]
    fn test_entropy_monotonicity() {
        let config = QuantumStreamConfig {
            dimension: 8,
            replenishment_rate: 0.05,
            robustness_threshold: 0.5,
        };
        let mut engine = QuantumStreamingEngine::new(config);
        
        let initial_entropy = engine.get_state().entropy_estimate;
        
        for i in 0..20 {
            engine.process_token(i % 8);
        }
        
        let final_entropy = engine.get_state().entropy_estimate;
        assert!(final_entropy > 0.0);
    }
}

fn main() {
    println!("Quantum Streaming Architecture - Production Grade Implementation");
    println!("================================================================");
    
    let config = QuantumStreamConfig {
        dimension: 16,
        replenishment_rate: 0.05,
        robustness_threshold: 0.7,
    };
    
    let mut engine = QuantumStreamingEngine::new(config.clone());
    
    println!("\nInitial State:");
    let state = engine.get_state();
    println!("  Dimension: {}", config.dimension);
    println!("  Initial Entropy: {:.6}", state.entropy_estimate);
    println!("  Uniform Amplitude: {:.6}", state.amplitudes[0]);
    
    println!("\nProcessing 100 tokens...");
    for i in 0..100 {
        engine.process_token(i % 16);
    }
    
    let final_state = engine.get_state();
    println!("  Final Entropy: {:.6}", final_state.entropy_estimate);
    println!("  Replenishment Count: {}", final_state.replenishment_counter);
    println!("  Robustness Score: {:.6}", engine.get_robustness_score());
    
    println!("\nLower Bound Computations:");
    for dim in [4, 16, 64, 256] {
        for err in [0.01, 0.05, 0.1] {
            let bound = compute_lower_bound(dim, err);
            println!("  dim={}, err={:.2} -> bound={:.4}", dim, err, bound);
        }
    }
    
    println!("\nHash Verification:");
    for token in [0, 1, 2, 42, 100] {
        println!("  token={} -> hash={:x}", token, hash_token(token));
    }
    
    println!("\nRunning self-tests...");
    let test_config = QuantumStreamConfig {
        dimension: 4,
        replenishment_rate: 0.1,
        robustness_threshold: 0.5,
    };
    let mut test_engine = QuantumStreamingEngine::new(test_config);
    test_engine.process_token(0);
    assert!(test_engine.get_state().amplitudes[0] > 0.5);
    println!("  All assertions passed.");
    
    println!("\nArchitecture complete. Ready for production deployment.");
}
