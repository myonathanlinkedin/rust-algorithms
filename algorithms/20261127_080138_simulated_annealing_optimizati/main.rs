mod core;
use core::{Graph, SaConfig, SimulatedAnnealing};

fn test_small_graph() {
    // Complete graph of 4 vertices with known optimal TSP tour cost = 10.0
    // Edge weights:
    // 0-1: 1, 0-2: 4, 0-3: 6
    // 1-2: 2, 1-3: 5
    // 2-3: 3
    let weight = vec![
        vec![0.0, 1.0, 4.0, 6.0],
        vec![1.0, 0.0, 2.0, 5.0],
        vec![4.0, 2.0, 0.0, 3.0],
        vec![6.0, 5.0, 3.0, 0.0],
    ];
    let graph = Graph::new(weight);
    let config = SaConfig {
        initial_temp: 500.0,
        cooling_rate: 0.90,
        iterations_per_temp: 2000,
        min_temp: 1e-4,
        seed: 12345,
    };
    let sa = SimulatedAnnealing::new(graph, config);
    let (best_path, best_cost) = sa.run();

    // Known optimal tours (up to rotation/reversal) have cost 10.0:
    // 0-1-2-3-0 => 1+2+3+6 = 12 (not optimal)
    // 0-1-3-2-0 => 1+5+3+4 = 13
    // 0-2-1-3-0 => 4+2+5+6 = 17
    // The true optimum for this symmetric matrix is 10.0 via 0-1-2-3-0? Wait compute:
    // Let's compute manually: 0-1 (1) + 1-2 (2) + 2-3 (3) + 3-0 (6) = 12.
    // 0-2 (4) + 2-1 (2) + 1-3 (5) + 3-0 (6) = 17.
    // 0-3 (6) + 3-2 (3) + 2-1 (2) + 1-0 (1) = 12.
    // Actually the minimal possible is 12. We'll assert <= 12.0 + epsilon.
    let epsilon = 1e-6;
    assert!(best_cost <= 12.0 + epsilon,
            "Best cost {} exceeds known optimum 12.0", best_cost);
    // Verify that path is a permutation of 0..4
    let mut sorted = best_path.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, vec![0,1,2,3], "Path is not a permutation");
}

fn test_single_vertex() {
    let weight = vec![vec![0.0]];
    let graph = Graph::new(weight);
    let config = SaConfig::default();
    let sa = SimulatedAnnealing::new(graph, config);
    let (best_path, best_cost) = sa.run();
    assert_eq!(best_path, vec![0]);
    assert!((best_cost - 0.0).abs() < 1e-9);
}

fn test_two_vertices() {
    let weight = vec![
        vec![0.0, 7.0],
        vec![7.0, 0.0],
    ];
    let graph = Graph::new(weight);
    let config = SaConfig {
        initial_temp: 100.0,
        cooling_rate: 0.85,
        iterations_per_temp: 500,
        min_temp: 1e-5,
        seed: 7,
    };
    let sa = SimulatedAnnealing::new(graph, config);
    let (best_path, best_cost) = sa.run();
    // Only two possible tours, both cost 14 (go and return)
    assert_eq!(best_path.len(), 2);
    assert!((best_cost - 14.0).abs() < 1e-6);
}

fn main() {
    test_small_graph();
    test_single_vertex();
    test_two_vertices();
    // Simple sanity output
    println!("All simulated annealing tests passed.");
}