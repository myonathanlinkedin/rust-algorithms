pub struct BipartiteGraph {
    pub n_left: usize,
    pub n_right: usize,
    // adjacency list from left vertices (0..n_left) to right vertices (0..n_right)
    pub adj: Vec<Vec<usize>>,

}

impl BipartiteGraph {
    /// Creates a new bipartite graph.
    /// Panics if `adj.len()` != `n_left` or any right vertex index is out of bounds.
    pub fn new(n_left: usize, n_right: usize, adj: Vec<Vec<usize>>) -> Self {
        assert_eq!(adj.len(), n_left, "Adjacency list length must equal n_left");
        for (u, neighbors) in adj.iter().enumerate() {
            for &v in neighbors {
                assert!(
                    v < n_right,
                    "Right vertex index out of bounds in adjacency of left vertex {}",
                    u
                );
            }
        }
        BipartiteGraph {
            n_left,
            n_right,
            adj,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchingResult {
    pub matching_size: usize,
    // pair_u[u] = Some(v) if left vertex u is matched to right vertex v
    pub pair_u: Vec<Option<usize>>,
    // pair_v[v] = Some(u) if right vertex v is matched to left vertex u
    pub pair_v: Vec<Option<usize>>,

}