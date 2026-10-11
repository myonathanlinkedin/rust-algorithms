mod core;
use core::Matrix;

fn test_svd_reconstruction() {
    // Simple 2x2 matrix with known SVD
    let a = Matrix::new(vec![
        vec![3.0, 1.0],
        vec![1.0, 3.0],
    ]);
    let (u, sigma, vt) = a.svd();

    // Reconstruct A_hat = U * Σ * V^T
    let sigma_mat = {
        let mut s = vec![vec![0.0; sigma.len()]; sigma.len()];
        for i in 0..sigma.len() {
            s[i][i] = sigma[i];
        }
        Matrix::new(s)
    };
    let a_hat = u.multiply(&sigma_mat).multiply(&vt);
    // Verify reconstruction error is tiny
    for i in 0..a.rows() {
        for j in 0..a.cols() {
            let diff = (a.get(i, j) - a_hat.get(i, j)).abs();
            assert!(diff < 1e-8, "Reconstruction error too large at ({}, {}): {}", i, j, diff);
        }
    }
}

fn test_low_rank_approx() {
    // 3x3 matrix of rank 3
    let a = Matrix::new(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 10.0],
    ]);
    // Rank-2 approximation
    let a2 = a.low_rank_approx(2);
    // Compute Frobenius norm of difference
    let mut err = 0.0;
    for i in 0..a.rows() {
        for j in 0..a.cols() {
            let d = a.get(i, j) - a2.get(i, j);
            err += d * d;
        }
    }
    let frob = err.sqrt();
    // Error should be less than original matrix norm but greater than zero
    assert!(frob > 0.0, "Error should be positive");
    // Compare with full reconstruction error (should be larger)
    let a_full = a.low_rank_approx(3);
    let mut err_full = 0.0;
    for i in 0..a.rows() {
        for j in 0..a.cols() {
            let d = a.get(i, j) - a_full.get(i, j);
            err_full += d * d;
        }
    }
    let frob_full = err_full.sqrt();
    assert!(frob < frob_full + 1e-12, "Rank-2 error should be less than full error");
}

fn main() {
    test_svd_reconstruction();
    test_low_rank_approx();
    // Simple sanity print
    let a = Matrix::new(vec![
        vec![2.0, 0.0, 0.0],
        vec![0.0, 3.0, 4.0],
        vec![0.0, 4.0, 9.0],
    ]);
    let approx = a.low_rank_approx(2);
    println!("Original matrix: {:?}", a);
    println!("Rank-2 approximation: {:?}", approx);
}