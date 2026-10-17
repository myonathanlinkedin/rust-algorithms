mod core;
use core::{Matrix, svd_low_rank};

fn reconstruct(u: &Matrix, s: &Vec<f64>, v: &Matrix) -> Matrix {
    let mut recon = Matrix::new(vec![vec![0.0; v.cols()]; u.rows()]);
    for i in 0..s.len() {
        let sigma = s[i];
        let ui = u.column(i);
        let vi = v.column(i);
        let outer = Matrix::outer(&ui, &vi).scale(sigma);
        recon = recon + outer;
    }
    recon
}

fn main() {
    // Simple sanity check
    let a = Matrix::new(vec![
        vec![3.0, 2.0, 2.0],
        vec![2.0, 3.0, -2.0],
        vec![2.0, -2.0, 3.0],
    ]);
    let k = 2;
    let (u, s, v) = svd_low_rank(&a, k, 500, 1e-12);
    let recon = reconstruct(&u, &s, &v);
    let err = (a - recon).frobenius_norm();
    assert!(err < 1e-6);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_low_rank_approximation() {
        let a = Matrix::new(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0, 0.0],
            vec![0.0, 0.0, 3.0, 0.0],
            vec![0.0, 0.0, 0.0, 4.0],
        ]);
        // Rank‑2 approximation should keep the two largest singular values (4 and 3)
        let (u, s, v) = svd_low_rank(&a, 2, 500, 1e-12);
        let recon = reconstruct(&u, &s, &v);
        // Expected error = sqrt(1^2 + 2^2) = sqrt(5)
        let expected_err = (1.0_f64.powi(2) + 2.0_f64.powi(2)).sqrt();
        let err = (a - recon).frobenius_norm();
        assert!((err - expected_err).abs() < 1e-6);
    }

    #[test]
    fn test_full_rank_reconstruction() {
        let a = Matrix::new(vec![
            vec![5.0, 2.0],
            vec![2.0, 1.0],
        ]);
        let (u, s, v) = svd_low_rank(&a, 2, 500, 1e-12);
        let recon = reconstruct(&u, &s, &v);
        let err = (a - recon).frobenius_norm();
        assert!(err < 1e-6);
    }
}
