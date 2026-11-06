mod core;
use core::{Graph, SimulatedAnnealing};

fn main() {
    // Example distance matrix for 4 cities
    let dist = vec![
        vec![0.0, 2.0, 9.0, 10.0],
        vec![2.0, 0.0, 6.0, 4.0],
        vec![9.0, 6.0, 0.0, 8.0],
        vec![10.0, 4.0, 8.0, 0.0],
    ];
    let graph = Graph::new(dist);
    let mut sa = SimulatedAnnealing::new(graph, 42, 100.0, 0.001, 0.95, 100);
    let (tour, cost) = sa.run(10_000);
    println!("Best tour: {:?}, cost: {:.4}", tour, cost);
    // Known optimal tour cost for this instance is 21.0 (0-1-3-2-0)
    assert!(cost <= 21.0 + 1e-6);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_graph_cost() {
        let dist = vec![
            vec![0.0, 1.0, 2.0],
            vec![1.0, 0.0, 3.0],
            vec![2.0, 3.0, 0.0],
        ];
        let g = Graph::new(dist);
        let tour = vec![0, 1, 2];
        // 0->1 (1) + 1->2 (3) + 2->0 (2) = 6
        assert!((g.cost(&tour) - 6.0).abs() < 1e-12);
    }

    #[test]
    fn test_sa_improves_solution() {
        // Simple 3‑city triangle where optimal tour cost is 3.0
        let dist = vec![
            vec![0.0, 1.0, 2.0],
            vec![1.0, 0.0, 1.0],
            vec![2.0, 1.0, 0.0],
        ];
        let graph = Graph::new(dist);
        let mut sa = SimulatedAnnealing::new(graph, 7, 10.0, 0.0001, 0.9, 50);
        let (tour, cost) = sa.run(5_000);
        // Verify cost does not exceed optimal + epsilon
        assert!(cost <= 3.0 + 1e-6);
        // Ensure tour visits each city exactly once
        let mut visited = tour.clone();
        visited.sort_unstable();
        assert_eq!(visited, vec![0, 1, 2]);
    }

    #[test]
    fn test_rng_determinism() {
        let mut rng1 = core::SimpleRng::new(12345);
        let mut rng2 = core::SimpleRng::new(12345);
        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }
}