pub const RECURSION_THRESHOLD: usize = 16;

/// Transpose a matrix stored in row‑major order using a cache‑oblivious recursive algorithm.
///
/// * `src` – slice containing the source matrix elements.
/// * `rows` – number of rows in the source matrix.
/// * `cols` – number of columns in the source matrix.
///
/// Returns a new `Vec<T>` containing the transposed matrix (size `cols × rows`).
///
/// The algorithm works out‑of‑place: it allocates the destination buffer once and
/// fills it recursively, minimizing cache misses without any explicit cache size
/// parameters.
///
/// # Panics
///
/// Panics if `src.len() != rows * cols`.
pub fn transpose<T: Clone>(src: &[T], rows: usize, cols: usize) -> Vec<T> {
    assert_eq!(src.len(), rows * cols, "source slice length does not match dimensions");
    if rows == 0 || cols == 0 {
        return Vec::new();
    }
    let mut dst = vec![src[0].clone(); rows * cols]; // allocate with correct length
    // Destination dimensions are cols × rows
    transpose_recursive(
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
    );
    dst
}

/// Internal recursive routine.
///
/// `src` – source slice (row‑major, dimensions `src_rows × src_cols`).
/// `src_rows`, `src_cols` – dimensions of the current sub‑matrix.
/// `src_row_off`, `src_col_off` – offsets of the sub‑matrix within `src`.
///
/// `dst` – mutable destination slice (row‑major, dimensions `dst_rows × dst_cols`).
/// `dst_rows`, `dst_cols` – dimensions of the destination matrix (always the transpose of the original).
/// `dst_row_off`, `dst_col_off` – offsets of the sub‑matrix within `dst`.
pub fn transpose_recursive<T: Clone>(
    src: &[T],
    src_rows: usize,
    src_cols: usize,
    src_row_off: usize,
    src_col_off: usize,
    dst: &mut [T],
    dst_rows: usize,
    dst_cols: usize,
    dst_row_off: usize,
    dst_col_off: usize,
) {
    // Base case: small block – perform naive copy.
    if src_rows <= RECURSION_THRESHOLD && src_cols <= RECURSION_THRESHOLD {
        for r in 0..src_rows {
            for c in 0..src_cols {
                let src_idx = (src_row_off + r) * src_cols + (src_col_off + c);
                let dst_idx = (dst_row_off + c) * dst_cols + (dst_col_off + r);
                // SAFETY: indices are within bounds by construction.
                dst[dst_idx] = src[src_idx].clone();
            }
        }
        return;
    }

    // Choose split dimension: split the larger side to keep sub‑problems balanced.
    if src_rows >= src_cols {
        // Split rows
        let mid = src_rows / 2;
        // Upper part
        transpose_recursive(
            src,
            mid,
            src_cols,
            src_row_off,
            src_col_off,
            dst,
            dst_rows,
            dst_cols,
            dst_row_off,
            dst_col_off,
        );
        // Lower part
        transpose_recursive(
            src,
            src_rows - mid,
            src_cols,
            src_row_off + mid,
            src_col_off,
            dst,
            dst_rows,
            dst_cols,
            dst_row_off,
            dst_col_off + mid,
        );
    } else {
        // Split columns
        let mid = src_cols / 2;
        // Left part
        transpose_recursive(
            src,
            src_rows,
            mid,
            src_row_off,
            src_col_off,
            dst,
            dst_rows,
            dst_cols,
            dst_row_off,
            dst_col_off,
        );
        // Right part
        transpose_recursive(
            src,
            src_rows,
            src_cols - mid,
            src_row_off,
            src_col_off + mid,
            dst,
            dst_rows,
            dst_cols,
            dst_row_off + mid,
            dst_col_off,
        );
    }
}