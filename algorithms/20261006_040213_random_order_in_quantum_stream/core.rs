use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Clone, PartialEq)]
pub struct QuantumStreamConfig {
    pub dimension: usize,
    pub replenishment_rate: f64,
    pub robustness_threshold: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StreamState {
    pub amplitudes: Vec<f64>,
    pub phase: f64,
    pub entropy_estimate: f64,
    pub replenishment_counter: usize,
}

pub struct QuantumStreamingEngine {
    config: QuantumStreamConfig,
    state: StreamState,
    history: Vec<f64>,
}

impl QuantumStreamingEngine {
    pub fn new(config: QuantumStreamConfig) -> Self {
        let dim = config.dimension;
        let uniform_amp = 1.0 / (dim as f64).sqrt();
        let amplitudes = vec![uniform_amp; dim];
        
        let state = StreamState {
            amplitudes,
            phase: 0.0,
            entropy_estimate: (dim as f64).ln(),
            replenishment_counter: 0,
        };

        QuantumStreamingEngine {
            config,
            state,
            history: Vec::new(),
        }
    }

    pub fn process_token(&mut self, token: usize) {
        if token >= self.config.dimension {
            return;
        }

        let amp = self.state.amplitudes[token];
        let new_amp = (amp + self.config.replenishment_rate).min(1.0);
        self.state.amplitudes[token] = new_amp;

        let norm_sq: f64 = self.state.amplitudes.iter().map(|a| a * a).sum();
        if norm_sq > 0.0 {
            let norm = norm_sq.sqrt();
            for a in self.state.amplitudes.iter_mut() {
                *a /= norm;
            }
        }

        self.state.phase += 0.1;
        self.state.replenishment_counter += 1;

        let entropy = self.calculate_entropy();
        self.state.entropy_estimate = entropy;

        self.history.push(entropy);
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    fn calculate_entropy(&self) -> f64 {
        let mut entropy = 0.0;
        for &amp in &self.state.amplitudes {
            let prob = amp * amp;
            if prob > 1e-15 {
                entropy -= prob * prob.ln();
            }
        }
        entropy
    }

    pub fn get_robustness_score(&self) -> f64 {
        let max_amp = self.state.amplitudes.iter().cloned().fold(0.0, f64::max);
        let min_amp = self.state.amplitudes.iter().cloned().fold(f64::MAX, f64::min);
        
        let spread = max_amp - min_amp;
        let entropy_ratio = self.state.entropy_estimate / (self.config.dimension as f64).ln();
        
        (1.0 - spread) * entropy_ratio
    }

    pub fn get_state(&self) -> &StreamState {
        &self.state
    }

    pub fn get_config(&self) -> &QuantumStreamConfig {
        &self.config
    }

    pub fn reset(&mut self) {
        let dim = self.config.dimension;
        let uniform_amp = 1.0 / (dim as f64).sqrt();
        self.state.amplitudes = vec![uniform_amp; dim];
        self.state.phase = 0.0;
        self.state.entropy_estimate = (dim as f64).ln();
        self.state.replenishment_counter = 0;
        self.history.clear();
    }
}

pub fn compute_lower_bound(dimension: usize, error_rate: f64) -> f64 {
    if dimension == 0 || error_rate <= 0.0 || error_rate >= 1.0 {
        return 0.0;
    }
    
    let log_dim = (dimension as f64).ln();
    let error_term = -error_rate.ln();
    
    log_dim * error_term * 2.0
}

pub fn hash_token(token: usize) -> u64 {
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    hasher.finish()
}
