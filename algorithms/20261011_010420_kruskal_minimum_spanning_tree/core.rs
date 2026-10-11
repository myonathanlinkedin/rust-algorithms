pub mod core {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Edge {
        pub u: usize,
        pub v: usize,
        pub weight: u32,

    }

    pub struct DSU {
        pub parent: Vec<usize>,
        pub rank: Vec<usize>,

    }

    impl DSU {
        pub fn new(size: usize) -> Self {
            let mut parent = Vec::with_capacity(size);
            let rank = vec![0; size];
            for i in 0..size {
                parent.push(i);
            }
            DSU { parent, rank }
        }

        fn find(&mut self, x: usize) -> usize {
            if self.parent[x] != x {
                let root = self.find(self.parent[x]);
                self.parent[x] = root;
            }
            self.parent[x]
        }

        pub fn union(&mut self, a: usize, b: usize) -> bool {
            let mut root_a = self.find(a);
            let mut root_b = self.find(b);
            if root_a == root_b {
                return false;
            }
            // Union by rank
            if self.rank[root_a] < self.rank[root_b] {
                std::mem::swap(&mut root_a, &mut root_b);
            }
            self.parent[root_b] = root_a;
            if self.rank[root_a] == self.rank[root_b] {
                self.rank[root_a] += 1;
            }
            true
        }
    }

    /// Computes a Minimum Spanning Tree (or Forest) using Kruskal's algorithm.
    ///
    /// Returns a tuple containing the list of edges in the MST and the total weight.
    /// The algorithm works for any undirected weighted graph; if the graph is
    /// disconnected, the result is a minimum spanning forest.
    pub fn kruskal(num_vertices: usize, mut edges: Vec<Edge>) -> (Vec<Edge>, u32) {
        // Sort edges by non‑decreasing weight
        edges.sort_by_key(|e| e.weight);
        let mut dsu = DSU::new(num_vertices);
        let mut mst: Vec<Edge> = Vec::new();
        let mut total_weight: u32 = 0;

        for edge in edges.into_iter() {
            if dsu.union(edge.u, edge.v) {
                total_weight = total_weight.saturating_add(edge.weight);
                mst.push(edge);
                if mst.len() == num_vertices.saturating_sub(1) {
                    // Early exit: a spanning tree for a connected component is complete
                    // For a forest we continue until all edges are processed.
                }
            }
        }
        (mst, total_weight)
    }
}

// Re‑export for external visibility
pub use core::{Edge, kruskal};