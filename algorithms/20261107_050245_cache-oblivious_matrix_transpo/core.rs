pub fn transpose<T: Clone>(src: &[T], rows: usize, cols: usize) -> Vec<T> {
    assert_eq!(src.len(), rows * cols);
    if rows == 0 || cols == 0 {
        return Vec::new();
    }
    // Allocate destination vector; clone first element to fill
    let mut dst = vec![src[0].clone(); rows * cols];
    transpose_recursive(src, rows, cols, 0, 0, rows, cols, &mut dst);
    dst
}

// Recursive helper for cache‑oblivious transposition
pub fn transpose_recursive<T: Clone>(
    src: &[T],
    src_rows_total: usize,
    src_cols_total: usize,
    r0: usize,
    c0: usize,
    rows: usize,
    cols: usize,
    dst: &mut [T],
) {
    if rows == 0 || cols == 0 {
        return;
    }
    if rows == 1 && cols == 1 {
        let src_idx = r0 * src_cols_total + c0;
        let dst_idx = c0 * src_rows_total + r0; // destination dimensions are cols × rows
        dst[dst_idx] = src[src_idx].clone();
        return;
    }
    if rows >= cols {
        let mid = rows / 2;
        // Top half
        transpose_recursive(
            src,
            src_rows_total,
            src_cols_total,
            r0,
            c0,
            mid,
            cols,
            dst,
        );
        // Bottom half
        transpose_recursive(
            src,
            src_rows_total,
            src_cols_total,
            r0 + mid,
            c0,
            rows - mid,
            cols,
            dst,
        );
    } else {
        let mid = cols / 2;
        // Left half
        transpose_recursive(
            src,
            src_rows_total,
            src_cols_total,
            r0,
            c0,
            rows,
            mid,
            dst,
        );
        // Right half
        transpose_recursive(
            src,
            src_rows_total,
            src_cols_total,
            r0,
            c0 + mid,
            rows,
            cols - mid,
            dst,
        );
    }
}