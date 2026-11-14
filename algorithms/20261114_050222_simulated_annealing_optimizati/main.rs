mod core;
use core::{Graph, SimulatedAnnealing};

fn main() {
    // Example: asymmetric 4‑city distance matrix
    let distances = vec![
        vec![0.0, 2.0, 9.0, 10.0],
        vec![1.0, 0.0, 6.0, 4.0],
        vec![15.0, 7.0, 0.0, 8.0],
        vec![6.0, 3.0, 12.0, 0.0],
    ];
    let graph = Graph::new(distances);
    let mut sa = SimulatedAnnealing::new(&graph, 42, 1000.0, 0.95, 1000);
    let (tour, length) = sa.solve();
    println!("Best tour: {:?}, length: {:.4}", tour, length);
    // Simple sanity check – the tour must contain each vertex exactly once.
    let mut sorted = tour.clone();
    sorted.sort_unstable();
    assert_eq!(sorted, (0..graph.size()).collect::<Vec<_>>());
    // Length must be finite and positive.
    assert!(length > 0.0 && length.is_finite());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tour_length() {
        let distances = vec![
            vec![0.0, 1.0, 2.0],
            vec![1.0, 0.0, 1.0],
            vec![2.0, 1.0, 0.0],
        ];
        let graph = Graph::new(distances);
        let tour = vec![0, 1, 2];
        let len = graph.tour_length(&tour);
        let expected = 1.0 + 1.0 + 2.0; // 0→1,1→2,2→0
        assert!((len - expected).abs() < 1e-9);
    }

    #[test]
    fn test_sa_converges_small() {
        // 4‑node square where optimal tour length is 4.0
        let distances = vec![
            vec![0.0, 1.0, 2.0, 1.0],
            vec![1.0, 0.0, 1.0, 2.0],
            vec![2.0, 1.0, 0.0, 1.0],
            vec![1.0, 2.0, 1.0, 0.0],
        ];
        let graph = Graph::new(distances);
        let mut sa = SimulatedAnnealing::new(&graph, 12345, 10.0, 0.9, 500);
        let (tour, length) = sa.solve();
        // Known optimum: 0‑1‑2‑3‑0 (or any rotation/reversal)
        assert!((length - 4.0).abs() < 1e-6);
        // Verify permutation property
        let mut sorted = tour.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..4).collect::<Vec<_>>());
    }

    #[test]
    fn test_rng_determinism() {
        let mut rng1 = core::XorShift64::new(777);
        let mut rng2 = core::XorShift64::new(777);
        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }
}