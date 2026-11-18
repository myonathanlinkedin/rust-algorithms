use std::f64;

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleRng {
    pub seed: u64,

}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        SimpleRng { seed }
    }

    // Linear Congruential Generator (64‑bit)
    fn next_u64(&mut self) -> u64 {
        // Constants from Numerical Recipes
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.seed
    }

    pub fn next_usize(&mut self, bound: usize) -> usize {
        if bound == 0 {
            return 0;
        }
        (self.next_u64() as usize) % bound
    }

    pub fn next_f64(&mut self) -> f64 {
        // Produce a double in [0,1)
        const MAX: f64 = u64::MAX as f64;
        (self.next_u64() as f64) / (MAX + 1.0)
    }

    pub fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Graph {
    // adjacency list: for each vertex, list of (neighbor, weight)
    pub adj: Vec<Vec<(usize, f64)>>,

}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Graph {
            adj: vec![Vec::new(); num_vertices],
        }
    }

    pub fn vertices(&self) -> usize {
        self.adj.len()
    }

    pub fn add_edge(&mut self, u: usize, v: usize, weight: f64) {
        assert!(u < self.vertices() && v < self.vertices());
        self.adj[u].push((v, weight));
        self.adj[v].push((u, weight));
    }

    // Compute total cut weight for a given assignment
    pub fn cut_score(&self, assignment: &[bool]) -> f64 {
        let mut total = 0.0;
        for (u, edges) in self.adj.iter().enumerate() {
            for &(v, w) in edges.iter() {
                if u < v && assignment[u] != assignment[v] {
                    total += w;
                }
            }
        }
        total
    }

    // Compute the change in cut weight if vertex `v` is flipped
    pub fn delta_flip(&self, assignment: &[bool], v: usize) -> f64 {
        let mut delta = 0.0;
        let current_side = assignment[v];
        for &(nbr, w) in self.adj[v].iter() {
            if assignment[nbr] == current_side {
                // edge currently internal, will become crossing
                delta += w;
            } else {
                // edge currently crossing, will become internal
                delta -= w;
            }
        }
        delta
    }
}

// Simulated Annealing for Max‑Cut
pub fn simulated_annealing(
    graph: &Graph,
    rng: &mut SimpleRng,
    init_temp: f64,
    cooling_rate: f64,
    iter_per_temp: usize,
    min_temp: f64,
) -> (Vec<bool>, f64) {
    let n = graph.vertices();
    // Initial random assignment
    let mut assignment: Vec<bool> = (0..n).map(|_| rng.next_bool()).collect();
    let mut current_score = graph.cut_score(&assignment);
    let mut best_assignment = assignment.clone();
    let mut best_score = current_score;

    let mut temp = init_temp;
    while temp > min_temp {
        for _ in 0..iter_per_temp {
            let v = rng.next_usize(n);
            let delta = graph.delta_flip(&assignment, v);
            if delta > 0.0 || rng.next_f64() < f64::exp(delta / temp) {
                // Accept move
                assignment[v] = !assignment[v];
                current_score += delta;
                if current_score > best_score {
                    best_score = current_score;
                    best_assignment = assignment.clone();
                }
            }
            // else reject: keep assignment unchanged
        }
        temp *= cooling_rate;
    }

    (best_assignment, best_score)
}