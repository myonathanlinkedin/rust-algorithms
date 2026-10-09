use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>,

}

impl Matrix {
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Self {
        assert_eq!(data.len(), rows * cols, "Data length does not match dimensions");
        Self { rows, cols, data }
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.data[row * self.cols + col] = value;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompressedMatrix {
    pub rows: usize,
    pub cols: usize,
    pub a_sign: Vec<bool>, // sign of row latent factor
    pub b_sign: Vec<bool>, // sign of column latent factor
    pub a_scale: f32,      // magnitude of row latent factor
    pub b_scale: f32,      // magnitude of column latent factor

}