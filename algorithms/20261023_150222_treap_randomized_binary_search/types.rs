pub struct Node {
    pub key: i32,
    pub priority: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,

}
pub struct Treap {
    pub root: Option<Box<Node>>,
    pub rng_state: u64,

}

impl Treap {
    pub fn new() -> Self {
        Treap {
            root: None,
            rng_state: 0x1234_5678_ABCD_EF01,
        }
    }

    fn next_priority(&mut self) -> u32 {
        // Simple linear congruential generator for deterministic pseudo‑randomness
        self.rng_state = self
            .rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1);
        (self.rng_state >> 32) as u32
    }

    pub fn insert(&mut self, key: i32) {
        if self.contains(key) {
            return; // ignore duplicates
        }
        let priority = self.next_priority();
        let new_node = Box::new(Node {
            key,
            priority,
            left: None,
            right: None,
        });
        self.root = crate::engine::insert_node(self.root.take(), new_node);
    }

    pub fn erase(&mut self, key: i32) {
        self.root = crate::engine::erase_node(self.root.take(), key);
    }

    pub fn contains(&self, key: i32) -> bool {
        crate::engine::contains_node(&self.root, key)
    }

    pub fn inorder(&self) -> Vec<i32> {
        let mut vec = Vec::new();
        crate::engine::inorder_traverse(&self.root, &mut vec);
        vec
    }
}