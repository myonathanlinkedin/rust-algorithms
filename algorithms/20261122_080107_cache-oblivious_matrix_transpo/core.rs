pub fn transpose<T: Clone + Default>(src: &[T], rows: usize, cols: usize) -> Vec<T> {
    // Handle degenerate cases early.
    if rows == 0 || cols == 0 {
        return Vec::new();
    }
    // Destination matrix has dimensions cols x rows.
    let mut dst = vec![T::default(); rows * cols];
    transpose_rec(
        src,
        rows,
        cols,
        0,
        0,
        &mut dst,
        cols,
        rows,
        0,
        0,
        rows,
        cols,
    );
    dst
}

// Recursive helper implementing the cache‑oblivious strategy.
// `src_rows`/`src_cols` are the full dimensions of the source matrix.
// Offsets (`src_row`, `src_col`, `dst_row`, `dst_col`) locate the current sub‑matrix.
// `rows` and `cols` are the size of the current sub‑matrix.
pub fn transpose_rec<T: Clone + Default>(
    src: &[T],
    src_rows: usize,
    src_cols: usize,
    src_row: usize,
    src_col: usize,
    dst: &mut [T],
    dst_rows: usize,
    dst_cols: usize,
    dst_row: usize,
    dst_col: usize,
    rows: usize,
    cols: usize,
) {
    const THRESHOLD: usize = 64; // Tunable base case size.

    if rows * cols <= THRESHOLD {
        // Naïve copy for small blocks.
        for i in 0..rows {
            for j in 0..cols {
                let src_idx = (src_row + i) * src_cols + (src_col + j);
                // Destination is transposed: (j,i) relative to the sub‑matrix.
                let dst_idx = (dst_col + j) * dst_cols + (dst_row + i);
                dst[dst_idx] = src[src_idx].clone();
            }
        }
    } else if rows >= cols {
        // Split the larger dimension (rows) in half.
        let mid = rows / 2;
        // Upper half.
        transpose_rec(
            src,
            src_rows,
            src_cols,
            src_row,
            src_col,
            dst,
            dst_rows,
            dst_cols,
            dst_row,
            dst_col,
            mid,
            cols,
        );
        // Lower half.
        transpose_rec(
            src,
            src_rows,
            src_cols,
            src_row + mid,
            src_col,
            dst,
            dst_rows,
            dst_cols,
            dst_row,
            dst_col + mid,
            rows - mid,
            cols,
        );
    } else {
        // Split the larger dimension (cols) in half.
        let mid = cols / 2;
        // Left half.
        transpose_rec(
            src,
            src_rows,
            src_cols,
            src_row,
            src_col,
            dst,
            dst_rows,
            dst_cols,
            dst_row,
            dst_col,
            rows,
            mid,
        );
        // Right half.
        transpose_rec(
            src,
            src_rows,
            src_cols,
            src_row,
            src_col + mid,
            dst,
            dst_rows,
            dst_cols,
            dst_row + mid,
            dst_col,
            rows,
            cols - mid,
        );
    }
}