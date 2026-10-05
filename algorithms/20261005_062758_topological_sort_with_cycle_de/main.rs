use std::collections::HashMap;

fn topological_sort(adj: &Vec<Vec<usize>>) -> Result<Vec<usize>, Vec<usize>> {
    let n = adj.len();
    let mut state = vec![0u8; n]; // 0=unvisited,1=visiting,2=visited
    let mut order = Vec::with_capacity(n);
    let mut cycle = Vec::new();
    let mut stack = Vec::new();

    fn dfs(
        u: usize,
        adj: &Vec<Vec<usize>>,
        state: &mut Vec<u8>,
        order: &mut Vec<usize>,
        cycle: &mut Vec<usize>,
        stack: &mut Vec<usize>,
    ) -> bool {
        state[u] = 1;
        stack.push(u);
        for &v in &adj[u] {
            if state[v] == 0 {
                if dfs(v, adj, state, order, cycle, stack) {
                    return true;
                }
            } else if state[v] == 1 {
                // cycle found
                let idx = stack.iter().position(|&x| x == v).unwrap();
                cycle.extend_from_slice(&stack[idx..]);
                return true;
            }
        }
        state[u] = 2;
        stack.pop();
        order.push(u);
        false
    }

    for u in 0..n {
        if state[u] == 0 {
            if dfs(u, adj, &mut state, &mut order, &mut cycle, &mut stack) {
                return Err(cycle);
            }
        }
    }
    order.reverse();
    Ok(order)
}

fn is_topological(order: &[usize], adj: &Vec<Vec<usize>>) -> bool {
    let mut pos = HashMap::new();
    for (i, &v) in order.iter().enumerate() {
        pos.insert(v, i);
    }
    for u in 0..adj.len() {
        for &v in &adj[u] {
            if pos[&u] >= pos[&v] {
                return false;
            }
        }
    }
    true
}

fn main() {
    // Test 1: simple DAG
    let adj1 = vec![
        vec![1],    // 0 -> 1
        vec![2],    // 1 -> 2
        vec![3],    // 2 -> 3
        vec![],     // 3
    ];
    let res1 = topological_sort(&adj1).expect("Should be acyclic");
    assert!(is_topological(&res1, &adj1));

    // Test 2: disconnected graph
    let adj2 = vec![
        vec![1],    // 0 -> 1
        vec![],     // 1
        vec![3],    // 2 -> 3
        vec![],     // 3
    ];
    let res2 = topological_sort(&adj2).expect("Should be acyclic");
    assert!(is_topological(&res2, &adj2));

    // Test 3: cycle detection
    let adj3 = vec![
        vec![1],    // 0 -> 1
        vec![2],    // 1 -> 2
        vec![0],    // 2 -> 0
    ];
    let res3 = topological_sort(&adj3);
    assert!(res3.is_err());
    let cycle = res3.err().unwrap();
    // cycle should contain all three nodes
    let mut cycle_set: Vec<usize> = cycle.clone();
    cycle_set.sort_unstable();
    assert_eq!(cycle_set, vec![0, 1, 2]);

    // Test 4: self-loop
    let adj4 = vec![
        vec![0],    // 0 -> 0
    ];
    let res4 = topological_sort(&adj4);
    assert!(res4.is_err());
    let cycle4 = res4.err().unwrap();
    assert_eq!(cycle4, vec![0]);

    println!("All tests passed.");
}
