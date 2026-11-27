use crate::types::Graph;

/// Tarjan's algorithm for Strongly Connected Components.
///
/// Returns a vector of components, each component being a vector of vertex indices.
/// The order of components and the order of vertices inside a component are
/// implementation‑defined (the caller may sort them for deterministic comparison).
pub fn tarjan_scc(graph: &Graph) -> Vec<Vec<usize>> {
    let n = graph.vertex_count();
    let mut index: Vec<Option<usize>> = vec![None; n];
    let mut lowlink: Vec<usize> = vec![0; n];
    let mut on_stack: Vec<bool> = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();
    let mut current_index: usize = 0;
    let mut result: Vec<Vec<usize>> = Vec::new();

    // Recursive helper that explores vertex `v`.
    fn strongconnect(
        v: usize,
        graph: &Graph,
        index: &mut [Option<usize>],
        lowlink: &mut [usize],
        on_stack: &mut [bool],
        stack: &mut Vec<usize>,
        current_index: &mut usize,
        result: &mut Vec<Vec<usize>>,
    ) {
        // Set the depth index for `v`.
        index[v] = Some(*current_index);
        lowlink[v] = *current_index;
        *current_index += 1;
        stack.push(v);
        on_stack[v] = true;

        // Consider successors of `v`.
        for &w in &graph.adj[v] {
            if index[w].is_none() {
                // Successor `w` has not yet been visited; recurse on it.
                strongconnect(
                    w,
                    graph,
                    index,
                    lowlink,
                    on_stack,
                    stack,
                    current_index,
                    result,
                );
                lowlink[v] = lowlink[v].min(lowlink[w]);
            } else if on_stack[w] {
                // Successor `w` is in the current SCC.
                lowlink[v] = lowlink[v].min(index[w].unwrap());
            }
        }

        // If `v` is a root node, pop the stack and generate an SCC.
        if lowlink[v] == index[v].unwrap() {
            let mut component: Vec<usize> = Vec::new();
            loop {
                let w = stack.pop().expect("stack underflow during SCC extraction");
                on_stack[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            result.push(component);
        }
    }

    // Start a DFS at each vertex that has not yet been visited.
    for v in 0..n {
        if index[v].is_none() {
            strongconnect(
                v,
                graph,
                &mut index,
                &mut lowlink,
                &mut on_stack,
                &mut stack,
                &mut current_index,
                &mut result,
            );
        }
    }

    result
}