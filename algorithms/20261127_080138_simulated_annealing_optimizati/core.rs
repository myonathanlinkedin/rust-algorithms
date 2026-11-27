use std::f64;

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    // Symmetric matrix of edge weights; weight[i][j] == weight[j][i]
    pub weight: Vec<Vec<f64>>,

}

impl Graph {
    pub fn new(weight: Vec<Vec<f64>>) -> Self {
        // Validate matrix is square
        let n = weight.len();
        for row in &weight {
            assert_eq!(row.len(), n, "Weight matrix must be square");
        }
        Self { weight }
    }

    pub fn order(&self) -> usize {
        self.weight.len()
    }

    // Compute total length of a tour (cycle) visiting vertices in order `path`
    // The tour is assumed to return to the start vertex.
    pub fn tour_cost(&self, path: &[usize]) -> f64 {
        let n = self.order();
        assert_eq!(path.len(), n, "Path length must equal number of vertices");
        let mut cost = 0.0;
        for i in 0..n {
            let a = path[i];
            let b = path[(i + 1) % n];
            cost += self.weight[a][b];
        }
        cost
    }
}

// Simple deterministic linear congruential generator (LCG)
#[derive(Debug, Clone, PartialEq)]
pub struct LcgRng {
    pub state: u64,

}

impl LcgRng {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    // Returns a u32 in [0, 2^32)
    pub fn next_u32(&mut self) -> u32 {
        // Parameters from Numerical Recipes
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        (self.state >> 16) as u32
    }

    // Returns a f64 in [0,1)
    pub fn next_f64(&mut self) -> f64 {
        const MAX_U32: f64 = u32::MAX as f64;
        self.next_u32() as f64 / (MAX_U32 + 1.0)
    }

    // Returns a usize in [0, bound)
    pub fn gen_range(&mut self, bound: usize) -> usize {
        assert!(bound > 0, "bound must be > 0");
        (self.next_f64() * bound as f64) as usize
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SaConfig {
    pub initial_temp: f64,
    pub cooling_rate: f64,          // Must be in (0,1)
    pub iterations_per_temp: usize,
    pub min_temp: f64,
    pub seed: u64,

}

impl Default for SaConfig {
    fn default() -> Self {
        Self {
            initial_temp: 1000.0,
            cooling_rate: 0.95,
            iterations_per_temp: 1000,
            min_temp: 1e-3,
            seed: 42,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimulatedAnnealing {
    pub graph: Graph,
    pub config: SaConfig,

}

impl SimulatedAnnealing {
    pub fn new(graph: Graph, config: SaConfig) -> Self {
        assert!(config.cooling_rate > 0.0 && config.cooling_rate < 1.0,
                "cooling_rate must be in (0,1)");
        assert!(config.initial_temp > 0.0, "initial_temp must be > 0");
        assert!(config.min_temp > 0.0, "min_temp must be > 0");
        Self { graph, config }
    }

    // Generate an initial random permutation of vertices [0..n)
    fn initial_state(&self, rng: &mut LcgRng) -> Vec<usize> {
        let n = self.graph.order();
        let mut state: Vec<usize> = (0..n).collect();
        // Fisher–Yates shuffle
        for i in (1..n).rev() {
            let j = rng.gen_range(i + 1);
            state.swap(i, j);
        }
        state
    }

    // Produce a neighbor by swapping two distinct random positions
    fn neighbor(&self, current: &[usize], rng: &mut LcgRng) -> Vec<usize> {
        let n = current.len();
        let mut next = current.to_vec();
        let i = rng.gen_range(n);
        let mut j = rng.gen_range(n);
        while j == i {
            j = rng.gen_range(n);
        }
        next.swap(i, j);
        next
    }

    // Core annealing loop; returns (best_path, best_cost)
    pub fn run(&self) -> (Vec<usize>, f64) {
        let mut rng = LcgRng::new(self.config.seed);
        let mut current = self.initial_state(&mut rng);
        let mut current_cost = self.graph.tour_cost(&current);
        let mut best = current.clone();
        let mut best_cost = current_cost;

        let mut temp = self.config.initial_temp;

        while temp > self.config.min_temp {
            for _ in 0..self.config.iterations_per_temp {
                let candidate = self.neighbor(&current, &mut rng);
                let candidate_cost = self.graph.tour_cost(&candidate);
                let delta = candidate_cost - current_cost;

                if delta < 0.0 {
                    // Better solution
                    current = candidate;
                    current_cost = candidate_cost;
                    if current_cost < best_cost {
                        best = current.clone();
                        best_cost = current_cost;
                    }
                } else {
                    // Accept with probability exp(-delta / temp)
                    let prob = (-delta / temp).exp();
                    if rng.next_f64() < prob {
                        current = candidate;
                        current_cost = candidate_cost;
                    }
                }
            }
            temp *= self.config.cooling_rate;
        }

        (best, best_cost)
    }
}