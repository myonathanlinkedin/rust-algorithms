pub struct HopcroftKarp {
    pub n_left: usize,
    pub n_right: usize,
    pub adj: Vec<Vec<usize>>, // adjacency from left vertices to right vertices

}

impl HopcroftKarp {
    /// Create a new bipartite graph with `n_left` vertices on the left side
    /// and `n_right` vertices on the right side. Vertices are indexed from 0.
    pub fn new(n_left: usize, n_right: usize) -> Self {
        HopcroftKarp {
            n_left,
            n_right,
            adj: vec![Vec::new(); n_left],
        }
    }

    /// Add an undirected edge between left vertex `u` and right vertex `v`.
    /// Panics if indices are out of bounds.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        assert!(u < self.n_left, "left vertex out of bounds");
        assert!(v < self.n_right, "right vertex out of bounds");
        self.adj[u].push(v);
    }

    /// Compute a maximum matching using the Hopcroft‑Karp algorithm.
    /// Returns a tuple `(matching_size, left_match, right_match)` where
    /// `left_match[u]` is `Some(v)` if left vertex `u` is matched to right vertex `v`,
    /// and `right_match[v]` is `Some(u)` analogously.
    pub fn max_matching(&self) -> (usize, Vec<Option<usize>>, Vec<Option<usize>>) {
        let mut pair_u: Vec<Option<usize>> = vec![None; self.n_left];
        let mut pair_v: Vec<Option<usize>> = vec![None; self.n_right];
        let mut dist: Vec<usize> = vec![0; self.n_left];
        let mut matching = 0usize;

        loop {
            if !self.bfs(&pair_u, &pair_v, &mut dist) {
                break;
            }
            for u in 0..self.n_left {
                if pair_u[u].is_none() && self.dfs(u, &mut pair_u, &mut pair_v, &mut dist) {
                    matching += 1;
                }
            }
        }

        (matching, pair_u, pair_v)
    }

    // Breadth‑first search to build distance layers.
    fn bfs(
        &self,
        pair_u: &[Option<usize>],
        pair_v: &[Option<usize>],
        dist: &mut [usize],
    ) -> bool {
        use std::collections::VecDeque;
        let mut queue = VecDeque::new();

        for u in 0..self.n_left {
            if pair_u[u].is_none() {
                dist[u] = 0;
                queue.push_back(u);
            } else {
                dist[u] = usize::MAX;
            }
        }

        let mut found_free = false;

        while let Some(u) = queue.pop_front() {
            for &v in &self.adj[u] {
                if let Some(paired_u) = pair_v[v] {
                    // v is matched; consider the next left vertex.
                    if dist[paired_u] == usize::MAX {
                        dist[paired_u] = dist[u] + 1;
                        queue.push_back(paired_u);
                    }
                } else {
                    // Found an unmatched right vertex; there exists an augmenting path.
                    found_free = true;
                }
            }
        }

        found_free
    }

    // Depth‑first search to find augmenting paths respecting the layering.
    fn dfs(
        &self,
        u: usize,
        pair_u: &mut [Option<usize>],
        pair_v: &mut [Option<usize>],
        dist: &mut [usize],
    ) -> bool {
        for &v in &self.adj[u] {
            if let Some(paired_u) = pair_v[v] {
                if dist[paired_u] == dist[u] + 1 && self.dfs(paired_u, pair_u, pair_v, dist) {
                    pair_u[u] = Some(v);
                    pair_v[v] = Some(u);
                    return true;
                }
            } else {
                // Free right vertex, we can augment.
                pair_u[u] = Some(v);
                pair_v[v] = Some(u);
                return true;
            }
        }
        // No augmenting path from this vertex; mark as unreachable.
        dist[u] = usize::MAX;
        false
    }
}