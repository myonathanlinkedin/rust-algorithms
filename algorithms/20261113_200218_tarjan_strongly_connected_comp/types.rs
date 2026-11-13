pub struct Graph {
    pub adj: Vec<Vec<usize>>,

}

impl Graph {
    /// Creates a new directed graph with `n` vertices (0..n-1) and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
        }
    }

    /// Adds a directed edge from `src` to `dst`.
    ///
    /// # Panics
    ///
    /// Panics if either vertex index is out of bounds.
    pub fn add_edge(&mut self, src: usize, dst: usize) {
        assert!(src < self.adj.len(), "source vertex out of bounds");
        assert!(dst < self.adj.len(), "destination vertex out of bounds");
        self.adj[src].push(dst);
    }

    /// Returns the number of vertices in the graph.
    pub fn vertex_count(&self) -> usize {
        self.adj.len()
    }
}