pub struct Graph {
    pub adj: Vec<Vec<usize>>,

}

impl Graph {
    /// Creates a directed graph with `num_vertices` isolated vertices.
    pub fn new(num_vertices: usize) -> Self {
        Self {
            adj: vec![Vec::new(); num_vertices],
        }
    }

    /// Adds a directed edge `from -> to`. Silently ignores out‑of‑bounds indices.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.adj.len() && to < self.adj.len() {
            self.adj[from].push(to);
        }
    }

    /// Returns the number of vertices in the graph.
    pub fn vertex_count(&self) -> usize {
        self.adj.len()
    }
}