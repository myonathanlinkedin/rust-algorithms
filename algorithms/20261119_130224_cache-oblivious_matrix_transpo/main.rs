mod core;
use core::transpose;

fn naive_transpose<T: Clone>(src: &[T], rows: usize, cols: usize) -> Vec<T> {
    let mut dst = vec![src[0].clone(); rows * cols];
    for r in 0..rows {
        for c in 0..cols {
            let src_idx = r * cols + c;
            let dst_idx = c * rows + r;
            dst[dst_idx] = src[src_idx].clone();
        }
    }
    dst
}

fn generate_matrix(rows: usize, cols: usize) -> Vec<usize> {
    (0..rows * cols).collect()
}

fn assert_equal<T: PartialEq + std::fmt::Debug>(a: &[T], b: &[T]) {
    assert_eq!(a, b, "Matrices differ: left = {:?}, right = {:?}", a, b);
}

fn main() {
    // Simple sanity check
    let src = vec![1, 2, 3, 4];
    let transposed = transpose(&src, 2, 2);
    let expected = vec![1, 3, 2, 4];
    assert_equal(&transposed, &expected);
    // Larger random‑like test
    let rows = 37;
    let cols = 23;
    let src = generate_matrix(rows, cols);
    let transposed = transpose(&src, rows, cols);
    let expected = naive_transpose(&src, rows, cols);
    assert_equal(&transposed, &expected);
    println!("All sanity checks passed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_small() {
        let src = vec![1, 2, 3, 4];
        let out = transpose(&src, 2, 2);
        let expected = vec![1, 3, 2, 4];
        assert_equal(&out, &expected);
    }

    #[test]
    fn test_rectangular() {
        let rows = 3;
        let cols = 5;
        let src = generate_matrix(rows, cols);
        let out = transpose(&src, rows, cols);
        let expected = naive_transpose(&src, rows, cols);
        assert_equal(&out, &expected);
    }

    #[test]
    fn test_large_random() {
        let rows = 128;
        let cols = 64;
        let src = generate_matrix(rows, cols);
        let out = transpose(&src, rows, cols);
        let expected = naive_transpose(&src, rows, cols);
        assert_equal(&out, &expected);
    }

    #[test]
    fn test_zero_dimensions() {
        let src: Vec<u8> = vec![];
        let out = transpose(&src, 0, 0);
        assert!(out.is_empty());
    }

    #[test]
    #[should_panic(expected = "source slice length does not match dimensions")]
    fn test_mismatched_length() {
        let src = vec![1, 2, 3];
        // rows*cols = 2*2 = 4, but slice length is 3
        let _ = transpose(&src, 2, 2);
    }
}