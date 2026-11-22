mod core;
use core::transpose;

fn naive_transpose<T: Clone>(src: &[T], rows: usize, cols: usize) -> Vec<T> {
    let mut dst = vec![src[0].clone(); rows * cols];
    for i in 0..rows {
        for j in 0..cols {
            dst[j * rows + i] = src[i * cols + j].clone();
        }
    }
    dst
}

fn main() {
    // Simple sanity checks executed at runtime.
    let a: Vec<u32> = vec![];
    let t = transpose(&a, 0, 0);
    assert!(t.is_empty());

    let b = vec![42];
    let t = transpose(&b, 1, 1);
    assert_eq!(t, vec![42]);

    let c = vec![
        1, 2, 3,
        4, 5, 6,
    ]; // 2 x 3 matrix
    let t = transpose(&c, 2, 3);
    assert_eq!(t, vec![
        1, 4,
        2, 5,
        3, 6,
    ]);

    // Larger random matrix verification.
    let rows = 16;
    let cols = 12;
    let mut src = Vec::with_capacity(rows * cols);
    for i in 0..rows * cols {
        src.push(i as u64);
    }
    let t = transpose(&src, rows, cols);
    let expected = naive_transpose(&src, rows, cols);
    assert_eq!(t, expected);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let src: Vec<u8> = vec![];
        let out = transpose(&src, 0, 0);
        assert_eq!(out.len(), 0);
    }

    #[test]
    fn test_one_by_one() {
        let src = vec![7];
        let out = transpose(&src, 1, 1);
        assert_eq!(out, vec![7]);
    }

    #[test]
    fn test_square() {
        let src = vec![
            1, 2, 3,
            4, 5, 6,
            7, 8, 9,
        ]; // 3x3
        let out = transpose(&src, 3, 3);
        let expected = naive_transpose(&src, 3, 3);
        assert_eq!(out, expected);
    }

    #[test]
    fn test_rectangular_more_rows() {
        let src = vec![
            1, 2,
            3, 4,
            5, 6,
            7, 8,
        ]; // 4x2
        let out = transpose(&src, 4, 2);
        let expected = naive_transpose(&src, 4, 2);
        assert_eq!(out, expected);
    }

    #[test]
    fn test_rectangular_more_cols() {
        let src = vec![
            1, 2, 3, 4,
            5, 6, 7, 8,
        ]; // 2x4
        let out = transpose(&src, 2, 4);
        let expected = naive_transpose(&src, 2, 4);
        assert_eq!(out, expected);
    }

    #[test]
    fn test_large_random() {
        let rows = 64;
        let cols = 48;
        let mut src = Vec::with_capacity(rows * cols);
        for i in 0..rows * cols {
            src.push((i * 31 + 7) as u32);
        }
        let out = transpose(&src, rows, cols);
        let expected = naive_transpose(&src, rows, cols);
        assert_eq!(out, expected);
    }
}