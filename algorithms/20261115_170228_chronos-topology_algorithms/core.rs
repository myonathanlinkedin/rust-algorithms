pub mod core {
    use std::collections::VecDeque;

    #[derive(Debug, Clone, PartialEq)]
    pub struct ChronoGraph {
        pub n: usize,
        pub adj: Vec<Vec<usize>>,
        pub indegree: Vec<usize>,

    }

    impl ChronoGraph {
        /// Creates a new graph with `n` vertices numbered `0..n-1`.
        pub fn new(n: usize) -> Self {
            Self {
                n,
                adj: vec![Vec::new(); n],
                indegree: vec![0; n],
            }
        }

        /// Adds a directed edge `from -> to`. Panics if indices are out of bounds.
        pub fn add_edge(&mut self, from: usize, to: usize) {
            assert!(from < self.n && to < self.n, "vertex index out of bounds");
            self.adj[from].push(to);
            self.indegree[to] = self.indegree[to].saturating_add(1);
        }

        /// Returns a topological ordering of the vertices if the graph is acyclic.
        /// On cycle detection returns an error string.
        pub fn topological_order(&self) -> Result<Vec<usize>, &'static str> {
            let mut indeg = self.indegree.clone();
            let mut queue = VecDeque::new();

            for v in 0..self.n {
                if indeg[v] == 0 {
                    queue.push_back(v);
                }
            }

            let mut order = Vec::with_capacity(self.n);
            while let Some(v) = queue.pop_front() {
                order.push(v);
                for &nbr in &self.adj[v] {
                    indeg[nbr] = indeg[nbr].saturating_sub(1);
                    if indeg[nbr] == 0 {
                        queue.push_back(nbr);
                    }
                }
            }

            if order.len() == self.n {
                Ok(order)
            } else {
                Err("graph contains a cycle")
            }
        }

        /// Computes the earliest start time for each vertex assuming each edge
        /// represents a unit duration task. Returns a vector `times` where
        /// `times[v]` is the length of the longest path from any source to `v`.
        /// Errors if the graph is cyclic.
        pub fn earliest_start_times(&self) -> Result<Vec<usize>, &'static str> {
            let order = self.topological_order()?;
            let mut times = vec![0usize; self.n];

            for &v in &order {
                let cur = times[v];
                for &nbr in &self.adj[v] {
                    let candidate = cur.saturating_add(1);
                    if candidate > times[nbr] {
                        times[nbr] = candidate;
                    }
                }
            }

            Ok(times)
        }
    }
}