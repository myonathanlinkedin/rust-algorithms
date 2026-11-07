use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct CGSolver {
    pub matrix: Vec<Vec<f64>>,

}

impl CGSolver {
    /// Create a new CG solver with the given square matrix.
    /// Panics if the matrix is not square.
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
        let n = matrix.len();
        for row in &matrix {
            assert_eq!(row.len(), n, "Matrix must be square");
        }
        Self { matrix }
    }

    /// Solve the linear system A x = b using the Conjugate Gradient method.
    /// `tol` is the tolerance for the residual norm.
    /// `max_iter` limits the number of iterations.
    /// Returns the approximate solution vector.
    pub fn solve(&self, b: &Vec<f64>, tol: f64, max_iter: usize) -> Vec<f64> {
        let n = self.matrix.len();
        assert_eq!(b.len(), n, "Dimension mismatch between matrix and vector b");

        // Initial guess x = 0
        let mut x = vec![0.0; n];
        // r = b - A * x
        let mut r = b.clone();
        // p = r
        let mut p = r.clone();
        let mut rs_old = dot(&r, &r);

        if rs_old.sqrt() < tol {
            return x;
        }

        for _ in 0..max_iter {
            let Ap = mat_vec_mul(&self.matrix, &p);
            let alpha = rs_old / dot(&p, &Ap);
            // x = x + alpha * p
            vec_add_in_place(&mut x, &p, alpha);
            // r = r - alpha * Ap
            vec_sub_in_place(&mut r, &Ap, alpha);
            let rs_new = dot(&r, &r);
            if rs_new.sqrt() < tol {
                break;
            }
            let beta = rs_new / rs_old;
            // p = r + beta * p
            vec_add_in_place(&mut p, &r, beta);
            rs_old = rs_new;
        }
        x
    }
}

// Vector and matrix helper functions

pub fn dot(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

pub fn mat_vec_mul(mat: &Vec<Vec<f64>>, vec: &Vec<f64>) -> Vec<f64> {
    let n = mat.len();
    let mut result = vec![0.0; n];
    for i in 0..n {
        let mut sum = 0.0;
        for j in 0..n {
            sum += mat[i][j] * vec[j];
        }
        result[i] = sum;
    }
    result
}

pub fn vec_add_in_place(a: &mut Vec<f64>, b: &Vec<f64>, scale: f64) {
    for (ai, bi) in a.iter_mut().zip(b.iter()) {
        *ai += scale * bi;
    }
}

pub fn vec_sub_in_place(a: &mut Vec<f64>, b: &Vec<f64>, scale: f64) {
    for (ai, bi) in a.iter_mut().zip(b.iter()) {
        *ai -= scale * bi;
    }
}