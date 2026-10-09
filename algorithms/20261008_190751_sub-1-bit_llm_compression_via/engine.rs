use crate::types::{CompressedMatrix, Matrix};

pub fn compress(matrix: &Matrix) -> CompressedMatrix {
    if matrix.rows == 0 || matrix.cols == 0 {
        return CompressedMatrix {
            rows: matrix.rows,
            cols: matrix.cols,
            a_sign: Vec::new(),
            b_sign: Vec::new(),
            a_scale: 0.0,
            b_scale: 0.0,
        };
    }

    // Compute row means
    let mut a_vals = Vec::with_capacity(matrix.rows);
    for i in 0..matrix.rows {
        let mut sum = 0.0f32;
        for j in 0..matrix.cols {
            sum += matrix.get(i, j);
        }
        a_vals.push(sum / matrix.cols as f32);
    }

    // Compute column means
    let mut b_vals = Vec::with_capacity(matrix.cols);
    for j in 0..matrix.cols {
        let mut sum = 0.0f32;
        for i in 0..matrix.rows {
            sum += matrix.get(i, j);
        }
        b_vals.push(sum / matrix.rows as f32);
    }

    // Determine signs
    let a_sign: Vec<bool> = a_vals.iter().map(|&v| v >= 0.0).collect();
    let b_sign: Vec<bool> = b_vals.iter().map(|&v| v >= 0.0).collect();

    // Compute scales as mean absolute values
    let a_scale = a_vals.iter().map(|&v| v.abs()).sum::<f32>() / matrix.rows as f32;
    let b_scale = b_vals.iter().map(|&v| v.abs()).sum::<f32>() / matrix.cols as f32;

    CompressedMatrix {
        rows: matrix.rows,
        cols: matrix.cols,
        a_sign,
        b_sign,
        a_scale,
        b_scale,
    }
}

pub fn decompress(comp: &CompressedMatrix) -> Matrix {
    let mut data = Vec::with_capacity(comp.rows * comp.cols);
    for i in 0..comp.rows {
        let a_factor = if comp.a_sign[i] { 1.0 } else { -1.0 } * comp.a_scale;
        for j in 0..comp.cols {
            let b_factor = if comp.b_sign[j] { 1.0 } else { -1.0 } * comp.b_scale;
            data.push(a_factor * b_factor);
        }
    }
    Matrix::new(comp.rows, comp.cols, data)
}