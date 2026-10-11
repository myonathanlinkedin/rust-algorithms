use crate::main;
use crate::types::Matrix;

/// Naïve O(n³) matrix multiplication used for verification.
pub fn naive_mat_mul(a: &Matrix, b: &Matrix) -> Matrix {
    assert_eq!(a.cols, b.rows, "inner dimensions must agree");
    let mut result = Matrix::new(a.rows, b.cols);
    for i in 0..a.rows {
        for k in 0..a.cols {
            let a_ik = a.get(i, k);
            for j in 0..b.cols {
                let b_kj = b.get(k, j);
                let c_ij = result.get_mut(i, j);
                *c_ij += a_ik * b_kj;
            }
        }
    }
    result
}

/// Tile‑based matrix multiplication. `tile_size` determines the block dimension.
pub fn tiled_mat_mul(a: &Matrix, b: &Matrix, tile_size: usize) -> Matrix {
    assert_eq!(a.cols, b.rows, "inner dimensions must agree");
    let n = a.rows;
    let m = a.cols;
    let p = b.cols;
    let mut result = Matrix::new(n, p);

    // Iterate over tiles.
    let mut i0 = 0usize;
    while i0 < n {
        let i_max = usize::min(i0 + tile_size, n);
        let mut j0 = 0usize;
        while j0 < p {
            let j_max = usize::min(j0 + tile_size, p);
            let mut k0 = 0usize;
            while k0 < m {
                let k_max = usize::min(k0 + tile_size, m);
                // Compute the product of the (i0..i_max, k0..k_max) tile of A
                // with the (k0..k_max, j0..j_max) tile of B.
                for i in i0..i_max {
                    for k in k0..k_max {
                        let a_ik = a.get(i, k);
                        for j in j0..j_max {
                            let b_kj = b.get(k, j);
                            let c_ij = result.get_mut(i, j);
                            *c_ij += a_ik * b_kj;
                        }
                    }
                }
                k0 = k_max;
            }
            j0 = j_max;
        }
        i0 = i_max;
    }

    result
}
