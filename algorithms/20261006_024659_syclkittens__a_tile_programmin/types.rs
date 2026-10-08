use crate::main;

/// A simple dense matrix stored in row‑major order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<i64>,
}

impl Matrix {
    /// Creates a new matrix filled with zeros.
    pub fn new(rows: usize, cols: usize) -> Self {
        let size = rows.checked_mul(cols).expect("matrix size overflow");
        Self {
            rows,
            cols,
            data: vec![0i64; size],
        }
    }

    /// Creates a matrix from a flat vector. Panics if length does not match rows·cols.
    pub fn from_vec(rows: usize, cols: usize, data: Vec<i64>) -> Self {
        assert_eq!(data.len(), rows * cols, "data length does not match dimensions");
        Self { rows, cols, data }
    }

    /// Returns a reference to the element at (row, col).
    pub fn get(&self, row: usize, col: usize) -> i64 {
        let idx = self.index(row, col);
        self.data[idx]
    }

    /// Returns a mutable reference to the element at (row, col).
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut i64 {
        let idx = self.index(row, col);
        &mut self.data[idx]
    }

    /// Internal helper to compute linear index.
    #[inline(always)]
    fn index(&self, row: usize, col: usize) -> usize {
        assert!(row < self.rows, "row out of bounds");
        assert!(col < self.cols, "col out of bounds");
        row * self.cols + col
    }
}
