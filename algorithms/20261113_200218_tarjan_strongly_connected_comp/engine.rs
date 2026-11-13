use crate::types::Graph;

/// Result of Tarjan's algorithm: a vector of strongly connected components.
/// Each component is a vector of vertex indices. The components are ordered
/// in reverse topological order (i.e., a component appears before any component
/// it can reach).
pub fn tarjan_scc(graph: &Graph) -> Vec<Vec<usize>> {
    let n = graph.vertex_count();
    let mut index = vec![None; n];
    let mut lowlink = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();
    let mut current_index: usize = 0;
    let mut sccs: Vec<Vec<usize>> = Vec::new();

    fn strongconnect(
        v: usize,
        graph: &Graph,
        index: &mut [Option<usize>],
        lowlink: &mut [usize],
        on_stack: &mut [bool],
        stack: &mut Vec<usize>,
        current_index: &mut usize,
        sccs: &mut Vec<Vec<usize>>,
    ) {
        // Set the depth index for v to the smallest unused index
        index[v] = Some(*current_index);
        lowlink[v] = *current_index;
        *current_index += 1;
        stack.push(v);
        on_stack[v] = true;

        // Consider successors of v
        for &w in &graph.adj[v] {
            if index[w].is_none() {
                // Successor w has not yet been visited; recurse on it
                strongconnect(
                    w,
                    graph,
                    index,
                    lowlink,
                    on_stack,
                    stack,
                    current_index,
                    sccs,
                );
                lowlink[v] = lowlink[v].min(lowlink[w]);
            } else if on_stack[w] {
                // Successor w is in stack and hence in the current SCC
                lowlink[v] = lowlink[v].min(index[w].unwrap());
            }
        }

        // If v is a root node, pop the stack and generate an SCC
        if lowlink[v] == index[v].unwrap() {
            let mut component: Vec<usize> = Vec::new();
            loop {
                let w = stack.pop().expect("stack underflow");
                on_stack[w] = false;
                component.push(w);
                if w == v {
                    break;
                }
            }
            // Optional: sort each component for deterministic output
            component.sort_unstable();
            sccs.push(component);
        }
    }

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
                &mut sccs,
            );
        }
    }

    // The algorithm produces components in reverse topological order.
    // Reverse to obtain a more natural order (optional).
    sccs.reverse();
    sccs
}