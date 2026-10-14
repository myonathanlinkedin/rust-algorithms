pub struct Verge {
    pub n: usize,
    pub tree: Vec<i64>,

}

impl Verge {
    /// Creates a new Verge (Fenwick) tree for `size` elements, all initialized to zero.
    pub fn new(size: usize) -> Self {
        Verge {
            n: size,
            tree: vec![0; size + 1], // 1‑based internal indexing
        }
    }

    /// Adds `delta` to the element at position `idx` (0‑based).
    ///
    /// Time complexity: **O(log n)**
    pub fn add(&mut self, mut idx: usize, delta: i64) {
        // Convert to 1‑based index for the internal structure.
        idx += 1;
        while idx <= self.n {
            self.tree[idx] += delta;
            idx += lowbit(idx);
        }
    }

    /// Returns the prefix sum of the range `[0, idx)`.
    ///
    /// Time complexity: **O(log n)**
    pub fn prefix_sum(&self, mut idx: usize) -> i64 {
        // `idx` is already the count of elements, i.e. 1‑based exclusive bound.
        let mut res = 0i64;
        while idx > 0 {
            res += self.tree[idx];
            idx &= idx - 1; // clear lowest set bit
        }
        res
    }

    /// Returns the sum of the half‑open interval `[left, right)`.
    ///
    /// Panics if `left > right` or `right > n`.
    ///
    /// Time complexity: **O(log n)**
    pub fn range_sum(&self, left: usize, right: usize) -> i64 {
        assert!(left <= right, "left must not exceed right");
        assert!(right <= self.n, "right bound out of range");
        self.prefix_sum(right) - self.prefix_sum(left)
    }

    /// Retrieves the exact value stored at position `idx`.
    ///
    /// Time complexity: **O(log n)**
    pub fn get(&self, idx: usize) -> i64 {
        self.range_sum(idx, idx + 1)
    }
}

// Helper: lowest set bit (idx & -idx) for usize.
#[inline]
pub fn lowbit(x: usize) -> usize {
    x & (!x + 1)
}