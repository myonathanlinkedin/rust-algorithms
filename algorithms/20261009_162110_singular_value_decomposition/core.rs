use std::f64;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,

}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        // Ensure rectangular shape
        for row in &data {
            assert_eq!(row.len(), cols, "All rows must have the same length");
        }
        Matrix { data, rows, cols }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.data[i][j]
    }

    pub fn set(&mut self, i: usize, j: usize, val: f64) {
        self.data[i][j] = val;
    }

    pub fn transpose(&self) -> Matrix {
        let mut t = vec![vec![0.0; self.rows]; self.cols];
        for i in 0..self.rows {
            for j in 0..self.cols {
                t[j][i] = self.data[i][j];
            }
        }
        Matrix::new(t)
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows, "Incompatible dimensions for multiplication");
        let mut res = vec![vec![0.0; other.cols]; self.rows];
        for i in 0..self.rows {
            for k in 0..self.cols {
                let aik = self.data[i][k];
                for j in 0..other.cols {
                    res[i][j] += aik * other.data[k][j];
                }
            }
        }
        Matrix::new(res)
    }

    // Compute thin SVD using Jacobi eigen-decomposition of A^T A
    pub fn svd(&self) -> (Matrix, Vec<f64>, Matrix) {
        // Compute B = A^T * A (symmetric)
        let at = self.transpose();
        let b = at.multiply(self);
        // Eigen-decompose B
        let (eig_vectors, eig_values) = jacobi_eigen(&b);
        // Singular values are sqrt of eigenvalues (non-negative)
        let mut sigma = Vec::with_capacity(eig_values.len());
        for &val in &eig_values {
            sigma.push(if val > 0.0 { val.sqrt() } else { 0.0 });
        }
        // Sort singular values descending and permute vectors accordingly
        let mut idx: Vec<usize> = (0..sigma.len()).collect();
        idx.sort_by(|&i, &j| sigma[j].partial_cmp(&sigma[i]).unwrap());
        let mut sigma_sorted = Vec::with_capacity(sigma.len());
        let mut v_sorted = vec![vec![0.0; b.cols]; b.cols];
        for (new_i, &old_i) in idx.iter().enumerate() {
            sigma_sorted.push(sigma[old_i]);
            for row in 0..b.cols {
                v_sorted[row][new_i] = eig_vectors[row][old_i];
            }
        }
        let v = Matrix::new(v_sorted);
        // Compute U = A * V * Σ^{-1}
        let mut u_data = vec![vec![0.0; self.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    if sigma_sorted[k] > 1e-12 {
                        sum += self.get(i, k) * v.get(k, j) / sigma_sorted[k];
                    }
                }
                u_data[i][j] = sum;
            }
        }
        let u = Matrix::new(u_data);
        (u, sigma_sorted, v.transpose())
    }

    // Low-rank approximation using top k singular values/vectors
    pub fn low_rank_approx(&self, k: usize) -> Matrix {
        let (u, sigma, vt) = self.svd();
        let k = k.min(sigma.len());
        // Build Σ_k matrix
        let mut sigma_k = vec![vec![0.0; k]; k];
        for i in 0..k {
            sigma_k[i][i] = sigma[i];
        }
        let sigma_k = Matrix::new(sigma_k);
        // Compute U_k * Σ_k * V_k^T
        let u_k = Matrix::new(u.data.iter().map(|row| row[..k].to_vec()).collect());
        let vt_k = Matrix::new(vt.data.iter().map(|row| row[..k].to_vec()).collect());
        let us = u_k.multiply(&sigma_k);
        us.multiply(&vt_k)
    }
}

// Jacobi eigenvalue algorithm for symmetric matrices.
// Returns (eigenvectors, eigenvalues) where eigenvectors columns are eigenvectors.
pub fn jacobi_eigen(mat: &Matrix) -> (Vec<Vec<f64>>, Vec<f64>) {
    let n = mat.rows;
    assert_eq!(n, mat.cols, "Jacobi requires square matrix");
    let mut a = mat.data.clone();
    let mut v = identity(n);
    const EPS: f64 = 1e-12;
    const MAX_ITER: usize = 100;

    for _ in 0..MAX_ITER {
        // Find largest off-diagonal element
        let mut max_val = 0.0;
        let mut p = 0;
        let mut q = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                let val = a[i][j].abs();
                if val > max_val {
                    max_val = val;
                    p = i;
                    q = j;
                }
            }
        }
        if max_val < EPS {
            break;
        }
        // Compute rotation
        let app = a[p][p];
        let aqq = a[q][q];
        let apq = a[p][q];
        let phi = 0.5 * ((2.0 * apq) / (aqq - app)).atan();
        let c = phi.cos();
        let s = phi.sin();

        // Apply rotation to A
        for i in 0..n {
            if i != p && i != q {
                let aip = a[i][p];
                let aiq = a[i][q];
                a[i][p] = c * aip - s * aiq;
                a[p][i] = a[i][p];
                a[i][q] = s * aip + c * aiq;
                a[q][i] = a[i][q];
            }
        }
        let app_new = c * c * app - 2.0 * s * c * apq + s * s * aqq;
        let aqq_new = s * s * app + 2.0 * s * c * apq + c * c * aqq;
        a[p][p] = app_new;
        a[q][q] = aqq_new;
        a[p][q] = 0.0;
        a[q][p] = 0.0;

        // Apply rotation to eigenvector matrix V
        for i in 0..n {
            let vip = v[i][p];
            let viq = v[i][q];
            v[i][p] = c * vip - s * viq;
            v[i][q] = s * vip + c * viq;
        }
    }

    // Extract eigenvalues from diagonal
    let mut eigenvalues = Vec::with_capacity(n);
    for i in 0..n {
        eigenvalues.push(a[i][i]);
    }
    (v, eigenvalues)
}

// Helper to create identity matrix of size n
pub fn identity(n: usize) -> Vec<Vec<f64>> {
    let mut id = vec![vec![0.0; n]; n];
    for i in 0..n {
        id[i][i] = 1.0;
    }
    id
}