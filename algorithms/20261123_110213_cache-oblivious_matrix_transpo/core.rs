use std::cmp::min;

/// Transposes a matrix stored in row-major order.
/// `src` must contain `rows * cols` elements.
/// Returns a new vector containing the transposed matrix in row-major order.
///
/// This implementation uses a cache‑oblivious divide‑and‑conquer strategy.
/// It recursively splits the matrix into quadrants until the sub‑matrix
/// is small enough to copy directly.
pub fn transpose<T: Copy>(src: &[T], rows: usize, cols: usize) -> Vec<T> {
    // Handle empty matrices.
    if rows == 0 || cols == 0 {
        return Vec::new();
    }

    // Allocate destination buffer. We can safely use the first element of `src`
    // because `rows * cols > 0` guarantees that `src` is non‑empty.
    let mut dst = vec![src[0]; rows * cols];

    // The destination matrix has dimensions `cols x rows`.
    transpose_rec(
        src,
        rows,
        cols,
        &mut dst,
        cols,
        rows,
        0,
        0,
        0,
        0,
        rows,
        cols,
    );

    dst
}

// Recursive helper that copies a sub‑matrix from `src` to `dst`.
// `src_r`, `src_c` are the top‑left coordinates in the source matrix.
// `dst_r`, `dst_c` are the top‑left coordinates in the destination matrix.
// `r_size`, `c_size` are the dimensions of the sub‑matrix to copy.
pub fn transpose_rec<T: Copy>(
    src: &[T],
    src_rows: usize,
    src_cols: usize,
    dst: &mut [T],
    dst_rows: usize,
    dst_cols: usize,
    src_r: usize,
    src_c: usize,
    dst_r: usize,
    dst_c: usize,
    r_size: usize,
    c_size: usize,
) {
    // Base case: copy element‑wise when the sub‑matrix is small.
    if r_size <= 1 || c_size <= 1 {
        for i in 0..r_size {
            for j in 0..c_size {
                let src_idx = (src_r + i) * src_cols + (src_c + j);
                let dst_idx = (dst_r + j) * dst_cols + (dst_c + i);
                dst[dst_idx] = src[src_idx];
            }
        }
        return;
    }

    // Recursive case: split the sub‑matrix into four quadrants.
    let r_mid = r_size / 2;
    let c_mid = c_size / 2;

    // Top‑left quadrant
    transpose_rec(
        src,
        src_rows,
        src_cols,
        dst,
        dst_rows,
        dst_cols,
        src_r,
        src_c,
        dst_r,
        dst_c,
        r_mid,
        c_mid,
    );

    // Top‑right quadrant
    transpose_rec(
        src,
        src_rows,
        src_cols,
        dst,
        dst_rows,
        dst_cols,
        src_r,
        src_c + c_mid,
        dst_r + c_mid,
        dst_c,
        r_mid,
        c_size - c_mid,
    );

    // Bottom‑left quadrant
    transpose_rec(
        src,
        src_rows,
        src_cols,
        dst,
        dst_rows,
        dst_cols,
        src_r + r_mid,
        src_c,
        dst_r,
        dst_c + r_mid,
        r_size - r_mid,
        c_mid,
    );

    // Bottom‑right quadrant
    transpose_rec(
        src,
        src_rows,
        src_cols,
        dst,
        dst_rows,
        dst_cols,
        src_r + r_mid,
        src_c + c_mid,
        dst_r + c_mid,
        dst_c + r_mid,
        r_size - r_mid,
        c_size - c_mid,
    );
}