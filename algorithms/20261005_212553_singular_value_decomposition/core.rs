use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        assert!(!data.is_empty() && !data[0].is_empty());
        let cols = data[0].len();
        for row in &data {
            assert_eq!(row.len(), cols);
        }
        Matrix { data }
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data[0].len()
    }

    pub fn transpose(&self) -> Matrix {
        let mut t = vec![vec![0.0; self.rows()]; self.cols()];
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                t[j][i] = self.data[i][j];
            }
        }
        Matrix::new(t)
    }

    pub fn mul(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols(), other.rows());
        let mut res = vec![vec![0.0; other.cols()]; self.rows()];
        for i in 0..self.rows() {
            for k in 0..self.cols() {
                let aik = self.data[i][k];
                for j in 0..other.cols() {
                    res[i][j] += aik * other.data[k][j];
                }
            }
        }
        Matrix::new(res)
    }

    pub fn mul_vec(&self, v: &Vec<f64>) -> Vec<f64> {
        assert_eq!(self.cols(), v.len());
        let mut res = vec![0.0; self.rows()];
        for i in 0..self.rows() {
            let mut sum = 0.0;
            for j in 0..self.cols() {
                sum += self.data[i][j] * v[j];
            }
            res[i] = sum;
        }
        res
    }

    pub fn scale(&self, alpha: f64) -> Matrix {
        let mut s = self.clone();
        for row in &mut s.data {
            for x in row {
                *x *= alpha;
            }
        }
        s
    }

    pub fn sub_assign(&mut self, other: &Matrix) {
        assert_eq!(self.rows(), other.rows());
        assert_eq!(self.cols(), other.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }

    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().flatten().map(|x| x * x).sum::<f64>().sqrt()
    }

    pub fn column(&self, idx: usize) -> Vec<f64> {
        assert!(idx < self.cols());
        self.data.iter().map(|row| row[idx]).collect()
    }

    pub fn outer(u: &Vec<f64>, v: &Vec<f64>) -> Matrix {
        let mut m = vec![vec![0.0; v.len()]; u.len()];
        for i in 0..u.len() {
            for j in 0..v.len() {
                m[i][j] = u[i] * v[j];
            }
        }
        Matrix::new(m)
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.rows(), rhs.rows());
        assert_eq!(self.cols(), rhs.cols());
        let mut sum = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                sum.data[i][j] += rhs.data[i][j];
            }
        }
        sum
    }
}

impl Sub for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Matrix {
        assert_eq!(self.rows(), rhs.rows());
        assert_eq!(self.cols(), rhs.cols());
        let mut diff = self.clone();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                diff.data[i][j] -= rhs.data[i][j];
            }
        }
        diff
    }
}

fn l2_norm(v: &Vec<f64>) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn normalize(v: &mut Vec<f64>) {
    let n = l2_norm(v);
    if n > 0.0 {
        for x in v.iter_mut() {
            *x /= n;
        }
    }
}

// Power iteration for symmetric matrix (returns eigenvector, eigenvalue)
fn power_iteration_symmetric(mat: &Matrix, max_iter: usize, eps: f64) -> (Vec<f64>, f64) {
    let n = mat.cols();
    let mut b_k = vec![1.0; n];
    normalize(&mut b_k);
    let mut eigenvalue = 0.0;
    for _ in 0..max_iter {
        let b_k1 = mat.mul_vec(&b_k);
        let norm = l2_norm(&b_k1);
        if norm == 0.0 {
            break;
        }
        let mut b_k1_norm = b_k1.clone();
        for x in &mut b_k1_norm {
            *x /= norm;
        }
        let eigenvalue_next = b_k1_norm.iter().zip(b_k.iter()).map(|(a, b)| a * b).sum::<f64>();
        if (eigenvalue - eigenvalue_next).abs() < eps {
            b_k = b_k1_norm;
            eigenvalue = eigenvalue_next;
            break;
        }
        b_k = b_k1_norm;
        eigenvalue = eigenvalue_next;
    }
    (b_k, eigenvalue)
}

// Low‑rank SVD using successive dominant singular vectors
pub fn svd_low_rank(
    a: &Matrix,
    k: usize,
    max_iter: usize,
    eps: f64,
) -> (Matrix, Vec<f64>, Matrix) {
    let mut residual = a.clone();
    let mut us: Vec<Vec<f64>> = Vec::new();
    let mut vs: Vec<Vec<f64>> = Vec::new();
    let mut sigmas: Vec<f64> = Vec::new();

    for _ in 0..k {
        // Compute A^T A
        let ata = residual.transpose().mul(&residual);
        // Dominant eigenpair of A^T A
        let (v, lambda) = power_iteration_symmetric(&ata, max_iter, eps);
        if lambda <= eps {
            break;
        }
        let sigma = lambda.sqrt();
        // Compute corresponding left singular vector
        let av = residual.mul_vec(&v);
        let mut u = av.clone();
        for x in &mut u {
            *x /= sigma;
        }
        // Store components
        us.push(u.clone());
        vs.push(v.clone());
        sigmas.push(sigma);
        // Deflate residual: R = R - sigma * u * v^T
        let outer = Matrix::outer(&u, &v).scale(sigma);
        residual.sub_assign(&outer);
    }

    // Assemble U and V matrices (columns = components)
    let m = a.rows();
    let n = a.cols();
    let mut u_mat = vec![vec![0.0; us.len()]; m];
    for (col, vec_u) in us.iter().enumerate() {
        for row in 0..m {
            u_mat[row][col] = vec_u[row];
        }
    }
    let mut v_mat = vec![vec![0.0; vs.len()]; n];
    for (col, vec_v) in vs.iter().enumerate() {
        for row in 0..n {
            v_mat[row][col] = vec_v[row];
        }
    }

    (Matrix::new(u_mat), sigmas, Matrix::new(v_mat))
}
