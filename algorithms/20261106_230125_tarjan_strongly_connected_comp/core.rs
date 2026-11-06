pub struct Graph {
    /// Adjacency list where `adj[u]` contains all vertices v such that there is an edge u → v.
    pub adj: Vec<Vec<usize>>,
    /// Number of vertices in the graph.
    pub n: usize,

}

impl Graph {
    /// Creates a new directed graph with `n` vertices (0..n-1) and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
            n,
        }
    }

    /// Adds a directed edge from `src` to `dst`. Panics if vertices are out of bounds.
    pub fn add_edge(&mut self, src: usize, dst: usize) {
        assert!(src < self.n && dst < self.n, "vertex index out of bounds");
        self.adj[src].push(dst);
    }

    /// Returns all strongly connected components using Tarjan's algorithm.
    /// Each component is a vector of vertex indices. The order of components
    /// and the order of vertices inside a component are deterministic:
    /// vertices inside a component are sorted ascending, and components are
    /// sorted by the smallest vertex they contain.
    pub fn strongly_connected_components(&self) -> Vec<Vec<usize>> {
        let mut index: usize = 0;
        let mut indices = vec![None; self.n];
        let mut lowlink = vec![0usize; self.n];
        let mut on_stack = vec![false; self.n];
        let mut stack: Vec<usize> = Vec::new();
        let mut sccs: Vec<Vec<usize>> = Vec::new();

        fn strongconnect(
            v: usize,
            graph: &Graph,
            index: &mut usize,
            indices: &mut [Option<usize>],
            lowlink: &mut [usize],
            stack: &mut Vec<usize>,
            on_stack: &mut [bool],
            sccs: &mut Vec<Vec<usize>>,
        ) {
            // Set the depth index for v to the smallest unused index
            indices[v] = Some(*index);
            lowlink[v] = *index;
            *index += 1;
            stack.push(v);
            on_stack[v] = true;

            // Consider successors of v
            for &w in &graph.adj[v] {
                if indices[w].is_none() {
                    // Successor w has not yet been visited; recurse on it
                    strongconnect(
                        w, graph, index, indices, lowlink, stack, on_stack, sccs,
                    );
                    lowlink[v] = lowlink[v].min(lowlink[w]);
                } else if on_stack[w] {
                    // Successor w is in stack and hence in the current SCC
                    lowlink[v] = lowlink[v].min(indices[w].unwrap());
                }
            }

            // If v is a root node, pop the stack and generate an SCC
            if lowlink[v] == indices[v].unwrap() {
                let mut component: Vec<usize> = Vec::new();
                loop {
                    let w = stack.pop().expect("stack underflow");
                    on_stack[w] = false;
                    component.push(w);
                    if w == v {
                        break;
                    }
                }
                component.sort_unstable(); // deterministic order inside component
                sccs.push(component);
            }
        }

        for v in 0..self.n {
            if indices[v].is_none() {
                strongconnect(
                    v,
                    self,
                    &mut index,
                    &mut indices,
                    &mut lowlink,
                    &mut stack,
                    &mut on_stack,
                    &mut sccs,
                );
            }
        }

        // Sort components by their smallest vertex for deterministic overall order
        sccs.sort_by_key(|comp| comp[0]);
        sccs
    }
}