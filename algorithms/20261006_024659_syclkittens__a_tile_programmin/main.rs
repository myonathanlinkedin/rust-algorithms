mod types;
mod engine;

use types::Matrix;
use engine::{naive_mat_mul, tiled_mat_mul};

fn main() {
    // Simple sanity check with 2×2 matrices.
    let a = Matrix::from_vec(2, 2, vec![1, 2, 3, 4]); // [[1,2],[3,4]]
    let b = Matrix::from_vec(2, 2, vec![5, 6, 7, 8]); // [[5,6],[7,8]]
    let expected = naive_mat_mul(&a, &b);
    let tiled = tiled_mat_mul(&a, &b, 1);
    assert_eq!(expected, tiled, "tile size 1 should match naive result");

    // Larger random matrices.
    let n = 8usize;
    let m = 8usize;
    let p = 8usize;
    let mut data_a = Vec::with_capacity(n * m);
    let mut data_b = Vec::with_capacity(m * p);
    for i in 0..(n * m) {
        data_a.push((i as i64 * 3 + 7) % 13);
    }
    for i in 0..(m * p) {
        data_b.push((i as i64 * 5 + 11) % 17);
    }
    let a_big = Matrix::from_vec(n, m, data_a);
    let b_big = Matrix::from_vec(m, p, data_b);
    let naive = naive_mat_mul(&a_big, &b_big);
    let tiled = tiled_mat_mul(&a_big, &b_big, 4);
    assert_eq!(naive, tiled, "tile size 4 should match naive result");

    // Edge case: zero‑dimension matrices.
    let empty_a = Matrix::new(0, 5);
    let empty_b = Matrix::new(5, 0);
    let empty_res = tiled_mat_mul(&empty_a, &empty_b, 2);
    assert_eq!(empty_res.rows, 0);
    assert_eq!(empty_res.cols, 0);
    assert!(empty_res.data.is_empty());

    // Edge case: tile size larger than dimensions.
    let a_small = Matrix::from_vec(3, 3, vec![
        1, 2, 3,
        4, 5, 6,
        7, 8, 9,
    ]);
    let b_small = Matrix::from_vec(3, 3, vec![
        9, 8, 7,
        6, 5, 4,
        3, 2, 1,
    ]);
    let tiled_large = tiled_mat_mul(&a_small, &b_small, 10);
    let naive_small = naive_mat_mul(&a_small, &b_small);
    assert_eq!(tiled_large, naive_small, "tile size larger than matrix should still work");

    println!("All assertions passed.");
}
