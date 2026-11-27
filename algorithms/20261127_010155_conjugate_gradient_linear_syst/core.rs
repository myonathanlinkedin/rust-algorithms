use std::f64;

#[derive(Debug, Clone, PartialEq)]
pub struct ConjugateGradient {
    /// Symmetric positive‑definite matrix A (dense row‑major)
    pub a: Vec<Vec<f64>>,
    /// Right‑hand side vector b
    pub b: Vec<f64>,
    /// Desired residual norm tolerance
    pub tolerance: f64,
    /// Maximum number of iterations
    pub max_iter: usize,

}

impl ConjugateGradient {
    /// Create a new solver instance. Returns Err if dimensions are inconsistent.
    pub fn new(a: Vec<Vec<f64>>, b: Vec<f64>, tolerance: f64, max_iter: usize) -> Result<Self, &'static str> {
        let n = a.len();
        if n == 0 {
            return Err("Matrix A must be non‑empty");
        }
        if a.iter().any(|row| row.len() != n) {
            return Err("Matrix A must be square");
        }
        if b.len() != n {
            return Err("Vector b length must match dimensions of A");
        }
        if tolerance <= 0.0 {
            return Err("Tolerance must be positive");
        }
        if max_iter == 0 {
            return Err("max_iter must be greater than zero");
        }
        Ok(Self { a, b, tolerance, max_iter })
    }

    /// Solve Ax = b using the Conjugate Gradient method.
    /// Returns the solution vector x on success, or an error string on failure.
    pub fn solve(&self) -> Result<Vec<f64>, &'static str> {
        let n = self.b.len();
        let mut x = vec![0.0_f64; n]; // initial guess x₀ = 0
        let mut r = vec_sub(&self.b, &mat_vec_mul(&self.a, &x));
        let mut p = r.clone();
        let mut rs_old = dot(&r, &r);

        if rs_old.sqrt() < self.tolerance {
            return Ok(x);
        }

        for _iter in 0..self.max_iter {
            let ap = mat_vec_mul(&self.a, &p);
            let p_ap = dot(&p, &ap);
            if p_ap.abs() < f64::EPSILON {
                return Err("Breakdown: division by zero encountered");
            }
            let alpha = rs_old / p_ap;

            // x = x + alpha * p
            for i in 0..n {
                x[i] += alpha * p[i];
            }

            // r = r - alpha * ap
            for i in 0..n {
                r[i] -= alpha * ap[i];
            }

            let rs_new = dot(&r, &r);
            if rs_new.sqrt() < self.tolerance {
                return Ok(x);
            }

            let beta = rs_new / rs_old;
            // p = r + beta * p
            for i in 0..n {
                p[i] = r[i] + beta * p[i];
            }
            rs_old = rs_new;
        }
        Err("Maximum iterations exceeded without convergence")
    }
}

/// Compute y = A * x where A is a dense square matrix.
pub fn mat_vec_mul(a: &[Vec<f64>], x: &[f64]) -> Vec<f64> {
    let n = a.len();
    let mut y = vec![0.0_f64; n];
    for i in 0..n {
        let row = &a[i];
        let mut sum = 0.0_f64;
        for j in 0..n {
            sum += row[j] * x[j];
        }
        y[i] = sum;
    }
    y
}

/// Dot product of two vectors.
pub fn dot(v1: &[f64], v2: &[f64]) -> f64 {
    let mut acc = 0.0_f64;
    for i in 0..v1.len() {
        acc += v1[i] * v2[i];
    }
    acc
}

/// Element‑wise subtraction: a - b
pub fn vec_sub(a: &[f64], b: &[f64]) -> Vec<f64> {
    let mut res = vec![0.0_f64; a.len()];
    for i in 0..a.len() {
        res[i] = a[i] - b[i];
    }
    res
}