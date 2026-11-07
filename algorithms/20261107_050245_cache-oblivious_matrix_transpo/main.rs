mod core;

use core::transpose;

fn main() {
    // Simple sanity check
    let src = vec![1, 2, 3, 4, 5, 6];
    let rows = 2usize;
    let cols = 3usize;
    let trans = transpose(&src, rows, cols);
    let expected = vec![1, 4, 2, 5, 3, 6];
    assert_eq!(trans, expected);
    // Double transpose should recover the original matrix
    let back = transpose(&trans, cols, rows);
    assert_eq!(back, src);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_matrix() {
        let n = 4usize;
        let src: Vec<u32> = (0..(n * n) as u32).collect();
        let trans = transpose(&src, n, n);
        for i in 0..n {
            for j in 0..n {
                assert_eq!(src[i * n + j], trans[j * n + i]);
            }
        }
    }

    #[test]
    fn test_rectangular_more_rows() {
        let rows = 5usize;
        let cols = 3usize;
        let src: Vec<i32> = (0..(rows * cols) as i32).collect();
        let trans = transpose(&src, rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                assert_eq!(src[i * cols + j], trans[j * rows + i]);
            }
        }
    }

    #[test]
    fn test_rectangular_more_cols() {
        let rows = 2usize;
        let cols = 5usize;
        let src: Vec<i16> = (0..(rows * cols) as i16).collect();
        let trans = transpose(&src, rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                assert_eq!(src[i * cols + j], trans[j * rows + i]);
            }
        }
    }

    #[test]
    fn test_single_row() {
        let rows = 1usize;
        let cols = 7usize;
        let src: Vec<char> = "abcdefg".chars().collect();
        let trans = transpose(&src, rows, cols);
        for j in 0..cols {
            assert_eq!(src[j], trans[j * rows]);
        }
    }

    #[test]
    fn test_single_column() {
        let rows = 6usize;
        let cols = 1usize;
        let src: Vec<u8> = (10..16).collect();
        let trans = transpose(&src, rows, cols);
        for i in 0..rows {
            assert_eq!(src[i], trans[i * cols]);
        }
    }

    #[test]
    fn test_empty_matrix() {
        let src: Vec<u32> = Vec::new();
        let trans = transpose(&src, 0, 0);
        assert!(trans.is_empty());
    }

    #[test]
    fn test_double_transpose_identity() {
        let rows = 3usize;
        let cols = 4usize;
        let src: Vec<u8> = (0..(rows * cols) as u8).collect();
        let trans = transpose(&src, rows, cols);
        let back = transpose(&trans, cols, rows);
        assert_eq!(back, src);
    }
}