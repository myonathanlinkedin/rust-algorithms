#[derive(Debug, Clone, PartialEq)]
pub struct DSU {
    pub parent: Vec<usize>,
    pub rank: Vec<usize>,
    pub set_size: Vec<usize>,

}

impl DSU {
    /// Creates a new DSU with `n` isolated elements (0..n-1).
    pub fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let rank = vec![0; n];
        let set_size = vec![1; n];
        for i in 0..n {
            parent.push(i);
        }
        DSU { parent, rank, set_size }
    }

    /// Finds the representative of `x` with path compression.
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    /// Unites the sets containing `a` and `b`.
    /// Returns `true` if a merge happened, `false` if already in the same set.
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a == root_b {
            return false;
        }
        // Union by rank.
        if self.rank[root_a] < self.rank[root_b] {
            self.parent[root_a] = root_b;
            self.set_size[root_b] += self.set_size[root_a];
        } else if self.rank[root_a] > self.rank[root_b] {
            self.parent[root_b] = root_a;
            self.set_size[root_a] += self.set_size[root_b];
        } else {
            self.parent[root_b] = root_a;
            self.rank[root_a] += 1;
            self.set_size[root_a] += self.set_size[root_b];
        }
        true
    }

    /// Checks whether `a` and `b` belong to the same set.
    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// Returns the size of the set containing `x`.
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.set_size[root]
    }
}