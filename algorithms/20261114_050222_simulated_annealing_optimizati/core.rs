pub struct Graph {
    pub distances: Vec<Vec<f64>>,

}

impl Graph {
    pub fn new(distances: Vec<Vec<f64>>) -> Self {
        let n = distances.len();
        for row in &distances {
            assert_eq!(row.len(), n, "Distance matrix must be square");
        }
        Self { distances }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.distances.len()
    }

    #[inline]
    pub fn distance(&self, i: usize, j: usize) -> f64 {
        self.distances[i][j]
    }

    pub fn tour_length(&self, tour: &[usize]) -> f64 {
        let n = tour.len();
        if n == 0 {
            return 0.0;
        }
        let mut len = 0.0;
        for k in 0..n {
            let a = tour[k];
            let b = tour[(k + 1) % n];
            len += self.distance(a, b);
        }
        len
    }
}

// Simple XorShift64 PRNG – deterministic, no std::rand needed.
#[derive(Debug, Clone, PartialEq)]
pub struct XorShift64 {
    pub state: u64,

}

impl XorShift64 {
    pub fn new(seed: u64) -> Self {
        // Zero seed would lock the generator; avoid it.
        let init = if seed == 0 { 0xdead_beef_dead_beef } else { seed };
        Self { state: init }
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        // Produce a float in [0,1)
        const MAX_U64: f64 = u64::MAX as f64;
        (self.next_u64() as f64) / (MAX_U64 + 1.0)
    }

    #[inline]
    pub fn next_usize(&mut self, bound: usize) -> usize {
        assert!(bound > 0, "Bound must be positive");
        (self.next_f64() * (bound as f64)) as usize
    }
}

pub struct SimulatedAnnealing<'a> {
    pub graph: &'a Graph,
    pub rng: XorShift64,
    pub temperature: f64,
    pub cooling_rate: f64,
    pub inner_iters: usize,

}

impl<'a> SimulatedAnnealing<'a> {
    pub fn new(
        graph: &'a Graph,
        seed: u64,
        init_temp: f64,
        cooling_rate: f64,
        inner_iters: usize,
    ) -> Self {
        assert!(init_temp > 0.0, "Initial temperature must be positive");
        assert!(
            (0.0..1.0).contains(&cooling_rate),
            "Cooling rate must be in (0,1)"
        );
        assert!(inner_iters > 0, "Inner iterations must be positive");
        Self {
            graph,
            rng: XorShift64::new(seed),
            temperature: init_temp,
            cooling_rate,
            inner_iters,
        }
    }

    fn initial_tour(&mut self) -> Vec<usize> {
        let n = self.graph.size();
        let mut tour: Vec<usize> = (0..n).collect();
        // Fisher–Yates shuffle using our PRNG
        for i in (1..n).rev() {
            let j = self.rng.next_usize(i + 1);
            tour.swap(i, j);
        }
        tour
    }

    fn neighbor(tour: &mut [usize], rng: &mut XorShift64) {
        let n = tour.len();
        if n < 2 {
            return;
        }
        // Swap two distinct positions
        let i = rng.next_usize(n);
        let mut j = rng.next_usize(n - 1);
        if j >= i {
            j += 1;
        }
        tour.swap(i, j);
    }

    pub fn solve(&mut self) -> (Vec<usize>, f64) {
        let mut current = self.initial_tour();
        let mut best = current.clone();
        let mut current_len = self.graph.tour_length(&current);
        let mut best_len = current_len;

        while self.temperature > 1e-3 {
            for _ in 0..self.inner_iters {
                // Generate neighbor by swapping two cities
                let mut candidate = current.clone();
                Self::neighbor(&mut candidate, &mut self.rng);
                let candidate_len = self.graph.tour_length(&candidate);
                let delta = candidate_len - current_len;

                if delta < 0.0
                    || self.rng.next_f64()
                        < (-delta / self.temperature).exp()
                {
                    current = candidate;
                    current_len = candidate_len;
                    if current_len < best_len {
                        best = current.clone();
                        best_len = current_len;
                    }
                }
            }
            self.temperature *= self.cooling_rate;
        }

        (best, best_len)
    }
}