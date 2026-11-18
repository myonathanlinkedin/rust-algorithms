pub type Hash = u64;

#[derive(Debug, Clone, PartialEq)]
pub struct Proof {
    // (sibling_hash, is_left_sibling)
    pub siblings: Vec<(Hash, bool)>,

}