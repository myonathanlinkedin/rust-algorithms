mod core;
use core::ConjugateGradient;

/// Helper to compare two floating point vectors within a tolerance.
fn approx_eq(v1: &[f64], v2: &[f64], eps: f64) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for i in 0..v1.len() {
        if (v1[i] - v2[i]).abs() > eps {
            return false;
        }
    }
    true
}

fn main() {
    // Simple 2×2 SPD system:
    //   [4 1] [x0] = [1]
    //   [1 3] [x1]   [2]
    let a = vec![
        vec![4.0_f64, 1.0_f64],
        vec![1.0_f64, 3.0_f64],
    ];
    let b = vec![1.0_f64, 2.0_f64];
    let solver = ConjugateGradient::new(a, b, 1e-10, 1000).expect("valid parameters");
    let x = solver.solve().expect("solver should converge");

    // Expected solution: x = [0.090909..., 0.636363...]
    let expected = vec![0.09090909090909091_f64, 0.6363636363636364_f64];
    assert!(approx_eq(&x, &expected, 1e-8), "solution deviates: got {:?}, expected {:?}", x, expected);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use super::core::ConjugateGradient;

    #[test]
    fn test_identity_matrix() {
        let n = 5;
        let a = (0..n)
            .map(|i| {
                let mut row = vec![0.0_f64; n];
                row[i] = 1.0_f64;
                row
            })
            .collect::<Vec<_>>();
        let b = (0..n).map(|i| (i as f64) + 1.0).collect::<Vec<_>>();
        let solver = ConjugateGradient::new(a, b.clone(), 1e-12, 50).unwrap();
        let x = solver.solve().unwrap();
        assert!(approx_eq(&x, &b, 1e-10));
    }

    #[test]
    fn test_random_spd_small() {
        // Construct SPD matrix via MᵀM
        let m = vec![
            vec![2.0_f64, -1.0_f64, 0.0_f64],
            vec![-1.0_f64, 2.0_f64, -1.0_f64],
            vec![0.0_f64, -1.0_f64, 2.0_f64],
        ];
        // A = MᵀM (still SPD)
        let a = multiply_transpose(&m, &m);
        let b = vec![1.0_f64, 2.0_f64, 3.0_f64];
        let solver = ConjugateGradient::new(a, b.clone(), 1e-10, 100).unwrap();
        let x = solver.solve().unwrap();

        // Verify residual norm is below tolerance
        let residual = {
            let ax = mat_vec_mul(&solver.a, &x);
            vec_sub(&b, &ax)
        };
        let res_norm = dot(&residual, &residual).sqrt();
        assert!(res_norm < 1e-9);
    }

    #[test]
    fn test_invalid_dimensions() {
        let a = vec![vec![1.0_f64, 2.0_f64]];
        let b = vec![1.0_f64];
        let err = ConjugateGradient::new(a, b, 1e-6, 10).unwrap_err();
        assert_eq!(err, "Matrix A must be square");
    }

    // Helper: multiply transpose of left matrix with right matrix (both dense)
    fn multiply_transpose(left: &[Vec<f64>], right: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = left[0].len();
        let mut result = vec![vec![0.0_f64; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut sum = 0.0_f64;
                for k in 0..left.len() {
                    sum += left[k][i] * right[k][j];
                }
                result[i][j] = sum;
            }
        }
        result
    }

    // Re‑use core helpers for test verification
    fn mat_vec_mul(a: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
        super::mat_vec_mul(a, x)
    }
    fn dot(v1: &[f64], v2: &[f64]) -> f64 {
        super::dot(v1, v2)
    }
    fn vec_sub(a: &[f64], b: &[f64]) -> Vec<f64> {
        super::vec_sub(a, b)
    }
}