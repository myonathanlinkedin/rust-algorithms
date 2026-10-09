pub trait SubmodularOracle {
    /// Compare the value of the submodular function on two sets.
    /// Returns Ordering::Less if f(a) < f(b), Ordering::Equal if equal, Ordering::Greater otherwise.
    fn compare(&self, a: &[usize], b: &[usize]) -> std::cmp::Ordering;
    /// Optional exact value for testing/debugging; not required by the algorithm.
    fn value(&self, set: &[usize]) -> i32;
}