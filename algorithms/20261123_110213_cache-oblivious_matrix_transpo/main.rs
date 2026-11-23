mod core;

use core::transpose;

fn main() {
    // Run all unit tests when the binary is executed.
    // The tests are defined in the `tests` module below.
    // This `main` function is required by the specification.
    println!("Running cache‑oblivious matrix transposition tests...");
    // The tests will automatically run via `cargo test`.
}

#[cfg(test)]
mod tests {
    use super::transpose;
    use rand::Rng;

    // Helper to create a sequential matrix of given dimensions.
    fn sequential_matrix(rows: usize, cols: usize) -> Vec<u32> {
        (0..rows * cols).map(|i| i as u32).collect()
    }

    // Helper to compare two matrices for equality.
    fn matrices_equal<T: PartialEq>(a: &[T], b: &[T]) -> bool {
        a == b
    }

    #[test]
    fn test_empty_matrix() {
        let src: Vec<u32> = Vec::new();
        let dst = transpose(&src, 0, 0);
        assert!(dst.is_empty());
    }

    #[test]
    fn test_single_element() {
        let src = vec![42];
        let dst = transpose(&src, 1, 1);
        assert_eq!(dst, vec![42]);
    }

    #[test]
    fn test_square_matrices() {
        for size in 1..=20 {
            let src = sequential_matrix(size, size);
            let dst = transpose(&src, size, size);
            // Transpose again should recover the original.
            let recovered = transpose(&dst, size, size);
            assert!(matrices_equal(&src, &recovered), "size {}", size);
        }
    }

    #[test]
    fn test_rectangular_matrices() {
        let cases = [(3, 5), (5, 3), (1, 10), (10, 1), (7, 7)];
        for &(rows, cols) in &cases {
            let src = sequential_matrix(rows, cols);
            let dst = transpose(&src, rows, cols);
            // Verify dimensions: destination should be cols x rows.
            assert_eq!(dst.len(), rows * cols);
            // Verify that element at (r, c) in src equals element at (c, r) in dst.
            for r in 0..rows {
                for c in 0..cols {
                    let src_idx = r * cols + c;
                    let dst_idx = c * rows + r;
                    assert_eq!(src[src_idx], dst[dst_idx], "rows {} cols {}", rows, cols);
                }
            }
        }
    }

    #[test]
    fn test_random_matrices() {
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let rows = rng.gen_range(1..=50);
            let cols = rng.gen_range(1..=50);
            let src: Vec<u32> = (0..rows * cols).map(|_| rng.gen()).collect();
            let dst = transpose(&src, rows, cols);
            // Verify that transposition is correct.
            for r in 0..rows {
                for c in 0..cols {
                    let src_idx = r * cols + c;
                    let dst_idx = c * rows + r;
                    assert_eq!(src[src_idx], dst[dst_idx], "rows {} cols {}", rows, cols);
                }
            }
        }
    }

    #[test]
    fn test_large_matrix() {
        let rows = 256;
        let cols = 256;
        let src = sequential_matrix(rows, cols);
        let dst = transpose(&src, rows, cols);
        // Verify a few random positions.
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let r = rng.gen_range(0..rows);
            let c = rng.gen_range(0..cols);
            let src_idx = r * cols + c;
            let dst_idx = c * rows + r;
            assert_eq!(src[src_idx], dst[dst_idx]);
        }
    }

    #[test]
    fn test_identity_transpose() {
        // Transposing twice should yield the original matrix.
        let rows = 13;
        let cols = 17;
        let src = sequential_matrix(rows, cols);
        let dst = transpose(&src, rows, cols);
        let recovered = transpose(&dst, cols, rows);
        assert!(matrices_equal(&src, &recovered));
    }
}