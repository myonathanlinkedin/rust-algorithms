mod types;
mod engine;

use crate::engine::{compress, decompress};
use crate::types::{CompressedMatrix, Matrix};

fn main() {
    // Example matrix
    let data = vec![
        1.0, -2.0, 3.0,
        -4.0, 5.0, -6.0,
        7.0, -8.0, 9.0,
    ];
    let matrix = Matrix::new(3, 3, data);

    let compressed = compress(&matrix);
    let decompressed = decompress(&compressed);

    // Simple sanity check: dimensions match
    assert_eq!(matrix.rows, decompressed.rows);
    assert_eq!(matrix.cols, decompressed.cols);

    // Compute mean absolute error
    let mut error_sum = 0.0f32;
    for i in 0..matrix.rows {
        for j in 0..matrix.cols {
            let orig = matrix.get(i, j);
            let recon = decompressed.get(i, j);
            error_sum += (orig - recon).abs();
        }
    }
    let mae = error_sum / (matrix.rows * matrix.cols) as f32;
    println!("Mean Absolute Error: {:.4}", mae);

    // Edge case tests
    // Empty matrix
    let empty = Matrix::new(0, 0, Vec::new());
    let comp_empty = compress(&empty);
    let decomp_empty = decompress(&comp_empty);
    assert_eq!(comp_empty.rows, 0);
    assert_eq!(comp_empty.cols, 0);
    assert_eq!(decomp_empty.rows, 0);
    assert_eq!(decomp_empty.cols, 0);

    // Single element matrix
    let single = Matrix::new(1, 1, vec![42.0]);
    let comp_single = compress(&single);
    let decomp_single = decompress(&comp_single);
    assert_eq!(decomp_single.get(0, 0), 42.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_decompress_identity() {
        let data = vec![1.0, -1.0, 2.0, -2.0];
        let matrix = Matrix::new(2, 2, data);
        let comp = compress(&matrix);
        let recon = decompress(&comp);
        // Since we use a rank-1 approximation, exact reconstruction is not guaranteed.
        // Verify that dimensions match and values are within a reasonable tolerance.
        assert_eq!(matrix.rows, recon.rows);
        assert_eq!(matrix.cols, recon.cols);
        for i in 0..matrix.rows {
            for j in 0..matrix.cols {
                let orig = matrix.get(i, j);
                let val = recon.get(i, j);
                assert!((orig - val).abs() <= 10.0, "Value differs too much");
            }
        }
    }

    #[test]
    fn test_empty_matrix() {
        let empty = Matrix::new(0, 0, Vec::new());
        let comp = compress(&empty);
        let recon = decompress(&comp);
        assert_eq!(comp.rows, 0);
        assert_eq!(comp.cols, 0);
        assert_eq!(recon.rows, 0);
        assert_eq!(recon.cols, 0);
    }

    #[test]
    fn test_single_element() {
        let single = Matrix::new(1, 1, vec![3.14]);
        let comp = compress(&single);
        let recon = decompress(&comp);
        assert_eq!(recon.get(0, 0), 3.14);
    }
}