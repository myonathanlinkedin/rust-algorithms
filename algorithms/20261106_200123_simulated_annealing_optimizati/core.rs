pub struct Graph {
    pub n: usize,
    pub dist: Vec<Vec<f64>>,

}

impl Graph {
    pub fn new(dist: Vec<Vec<f64>>) -> Self {
        let n = dist.len();
        // Ensure matrix is square
        for row in &dist {
            assert_eq!(row.len(), n, "Distance matrix must be square");
        }
        Graph { n, dist }
    }

    pub fn cost(&self, tour: &[usize]) -> f64 {
        if tour.is_empty() {
            return 0.0;
        }
        let mut total = 0.0;
        for i in 0..tour.len() {
            let a = tour[i];
            let b = tour[(i + 1) % tour.len()];
            total += self.dist[a][b];
        }
        total
    }
}

// Simple deterministic linear congruential generator
pub struct SimpleRng {
    pub seed: u64,

}

impl SimpleRng {
    pub fn new(seed: u64) -> Self {
        SimpleRng { seed }
    }

    fn next_u64(&mut self) -> u64 {
        // Constants from Numerical Recipes
        const A: u64 = 1664525;
        const C: u64 = 1013904223;
        self.seed = self.seed.wrapping_mul(A).wrapping_add(C);
        self.seed
    }

    pub fn next_f64(&mut self) -> f64 {
        // Produce a float in [0,1)
        let bits = self.next_u64() >> 11; // keep 53 bits
        (bits as f64) * (1.0 / (1u64 << 53) as f64)
    }

    pub fn next_usize(&mut self, bound: usize) -> usize {
        if bound == 0 {
            return 0;
        }
        (self.next_u64() as usize) % bound
    }
}

pub struct SimulatedAnnealing {
    pub graph: Graph,
    pub rng: SimpleRng,
    pub temp_start: f64,
    pub temp_end: f64,
    pub alpha: f64,
    pub iter_per_temp: usize,

}

impl SimulatedAnnealing {
    pub fn new(
        graph: Graph,
        seed: u64,
        temp_start: f64,
        temp_end: f64,
        alpha: f64,
        iter_per_temp: usize,
    ) -> Self {
        assert!(temp_start > temp_end, "Start temperature must exceed end temperature");
        assert!(alpha > 0.0 && alpha < 1.0, "Alpha must be in (0,1)");
        SimulatedAnnealing {
            graph,
            rng: SimpleRng::new(seed),
            temp_start,
            temp_end,
            alpha,
            iter_per_temp,
        }
    }

    fn random_initial_tour(&mut self) -> Vec<usize> {
        let mut tour: Vec<usize> = (0..self.graph.n).collect();
        // Fisher–Yates shuffle
        for i in (1..tour.len()).rev() {
            let j = self.rng.next_usize(i + 1);
            tour.swap(i, j);
        }
        tour
    }

    fn neighbor(&mut self, tour: &mut Vec<usize>) {
        // Swap two distinct positions
        let n = tour.len();
        if n < 2 {
            return;
        }
        let i = self.rng.next_usize(n);
        let mut j = self.rng.next_usize(n);
        while j == i {
            j = self.rng.next_usize(n);
        }
        tour.swap(i, j);
    }

    pub fn run(&mut self, max_iters: usize) -> (Vec<usize>, f64) {
        let mut current = self.random_initial_tour();
        let mut best = current.clone();
        let mut current_cost = self.graph.cost(&current);
        let mut best_cost = current_cost;
        let mut temp = self.temp_start;
        let mut iter = 0usize;

        while temp > self.temp_end && iter < max_iters {
            for _ in 0..self.iter_per_temp {
                let mut candidate = current.clone();
                self.neighbor(&mut candidate);
                let candidate_cost = self.graph.cost(&candidate);
                let delta = candidate_cost - current_cost;
                if delta < 0.0 || self.rng.next_f64() < (-delta / temp).exp() {
                    current = candidate;
                    current_cost = candidate_cost;
                    if current_cost < best_cost {
                        best = current.clone();
                        best_cost = current_cost;
                    }
                }
                iter += 1;
                if iter >= max_iters {
                    break;
                }
            }
            temp *= self.alpha;
        }
        (best, best_cost)
    }
}