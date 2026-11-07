mod core;
use core::CGSolver;

fn main() {
    // Run unit tests
    test_small_system();
    test_convergence();
    test_tolerance_zero();
    test_dimension_mismatch();
    println!("All tests passed.");
}

fn test_small_system() {
    // Solve A x = b where A = [[4,1],[1,3]] and b = [1,2]
    let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
    let b = vec![1.0, 2.0];
    let solver = CGSolver::new(a);
    let x = solver.solve(&b, 1e-10, 100);
    // Expected solution: x = [0.090909..., 0.636363...]
    let expected = vec![0.09090909090909091, 0.6363636363636364];
    for (xi, ei) in x.iter().zip(expected.iter()) {
        assert!((xi - ei).abs() < 1e-6, "x = {:?}, expected = {:?}", x, expected);
    }
}

fn test_convergence() {
    // 3x3 SPD matrix
    let a = vec![
        vec![10.0, 2.0, 0.0],
        vec![2.0, 5.0, 1.0],
        vec![0.0, 1.0, 3.0],
    ];
    let b = vec![12.0, 7.0, 3.0];
    let solver = CGSolver::new(a);
    let x = solver.solve(&b, 1e-8, 50);
    // Verify residual norm
    let r = mat_vec_mul(&solver.matrix, &x);
    let residual: Vec<f64> = r.iter().zip(b.iter()).map(|(ri, bi)| ri - bi).collect();
    let res_norm = dot(&residual, &residual).sqrt();
    assert!(res_norm < 1e-8, "Residual norm too high: {}", res_norm);
}

fn test_tolerance_zero() {
    // When tol = 0, solver should perform max_iter iterations
    let a = vec![vec![2.0, 0.0], vec![0.0, 2.0]];
    let b = vec![2.0, 2.0];
    let solver = CGSolver::new(a);
    let x = solver.solve(&b, 0.0, 5);
    // Exact solution is [1,1]
    let expected = vec![1.0, 1.0];
    for (xi, ei) in x.iter().zip(expected.iter()) {
        assert!((xi - ei).abs() < 1e-6, "x = {:?}, expected = {:?}", x, expected);
    }
}

fn test_dimension_mismatch() {
    let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
    let b = vec![1.0];
    let solver = CGSolver::new(a);
    let result = std::panic::catch_unwind(|| solver.solve(&b, 1e-6, 10));
    assert!(result.is_err(), "Expected panic due to dimension mismatch");
}

// Helper functions for tests
fn dot(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn mat_vec_mul(mat: &Vec<Vec<f64>>, vec: &Vec<f64>) -> Vec<f64> {
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