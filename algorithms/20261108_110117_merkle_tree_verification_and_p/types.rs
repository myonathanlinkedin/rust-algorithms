pub type Hash = [u8; 32];

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleTree {
    // levels[0] = leaf hashes, levels[last] = root (single hash)
    pub levels: Vec<Vec<Hash>>,

}

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleProof {
    pub leaf_index: usize,
    pub sibling_hashes: Vec<(Direction, Hash)>,
    pub root: Hash,

}