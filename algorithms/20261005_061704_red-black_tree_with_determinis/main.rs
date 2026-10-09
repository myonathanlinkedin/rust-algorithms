use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Color {
    Red,
    Black,
}

struct Node<K, V> {
    key: K,
    val: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
    color: Color,
}

impl<K: Ord, V> Node<K, V> {
    fn new(key: K, val: V, color: Color) -> Self {
        Node {
            key,
            val,
            left: None,
            right: None,
            color,
        }
    }
}

fn is_red<K, V>(node: &Option<Box<Node<K, V>>>) -> bool {
    match node {
        Some(ref n) => n.color == Color::Red,
        None => false,
    }
}

fn rotate_left<K: Ord, V>(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let mut x = h.right.take().expect("rotate_left requires right child");
    h.right = x.left.take();
    x.left = Some(h);
    x.color = x.left.as_ref().unwrap().color;
    x.left.as_mut().unwrap().color = Color::Red;
    x
}

fn rotate_right<K: Ord, V>(mut h: Box<Node<K, V>>) -> Box<Node<K, V>> {
    let mut x = h.left.take().expect("rotate_right requires left child");
    h.left = x.right.take();
    x.right = Some(h);
    x.color = x.right.as_ref().unwrap().color;
    x.right.as_mut().unwrap().color = Color::Red;
    x
}

fn flip_colors<K, V>(h: &mut Box<Node<K, V>>) {
    h.color = match h.color {
        Color::Red => Color::Black,
        Color::Black => Color::Red,
    };
    if let Some(ref mut l) = h.left {
        l.color = match l.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }
    if let Some(ref mut r) = h.right {
        r.color = match r.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
    }
}

fn insert_node<K: Ord, V>(node: Option<Box<Node<K, V>>>, key: K, val: V) -> Box<Node<K, V>> {
    let mut h = match node {
        None => Box::new(Node::new(key, val, Color::Red)),
        Some(mut n) => {
            match key.cmp(&n.key) {
                Ordering::Less => n.left = Some(insert_node(n.left.take(), key, val)),
                Ordering::Greater => n.right = Some(insert_node(n.right.take(), key, val)),
                Ordering::Equal => n.val = val,
            }

            if is_red(&n.right) && !is_red(&n.left) {
                n = rotate_left(n);
            }
            if is_red(&n.left) && is_red(&n.left.as_ref().unwrap().left) {
                n = rotate_right(n);
            }
            if is_red(&n.left) && is_red(&n.right) {
                flip_colors(&mut n);
            }
            n
        }
    };
    h
}

pub struct RBTree<K, V> {
    root: Option<Box<Node<K, V>>>,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        RBTree { root: None }
    }

    pub fn insert(&mut self, key: K, val: V) {
        let root = self.root.take();
        let mut new_root = insert_node(root, key, val);
        new_root.color = Color::Black;
        self.root = Some(new_root);
        self.assert_valid();
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let mut cur = &self.root;
        while let Some(ref node) = cur {
            match key.cmp(&node.key) {
                Ordering::Less => cur = &node.left,
                Ordering::Greater => cur = &node.right,
                Ordering::Equal => return Some(&node.val),
            }
        }
        None
    }

    fn assert_valid(&self) {
        self.check_properties(&self.root, true);
    }

    fn check_properties(&self, node: &Option<Box<Node<K, V>>>, is_root: bool) -> usize {
        match node {
            None => 1,
            Some(ref n) => {
                if is_root && n.color != Color::Black {
                    panic!("Root must be black");
                }
                if n.color == Color::Red {
                    if is_red(&n.left) || is_red(&n.right) {
                        panic!("Red node cannot have red child");
                    }
                }
                let left_bh = self.check_properties(&n.left, false);
                let right_bh = self.check_properties(&n.right, false);
                if left_bh != right_bh {
                    panic!("Black-height mismatch");
                }
                left_bh + if n.color == Color::Black { 1 } else { 0 }
            }
        }
    }
}

fn main() {
    let mut tree = RBTree::new();
    tree.insert(10, "a");
    tree.insert(20, "b");
    tree.insert(5, "c");
    tree.insert(6, "d");
    tree.insert(15, "e");
    tree.insert(30, "f");
    tree.insert(25, "g");
    tree.insert(2, "h");
    tree.insert(8, "i");

    assert_eq!(tree.get(&10), Some(&"a"));
    assert_eq!(tree.get(&15), Some(&"e"));
    assert_eq!(tree.get(&2), Some(&"h"));
    assert_eq!(tree.get(&100), None);

    // Update existing key
    tree.insert(10, "z");
    assert_eq!(tree.get(&10), Some(&"z"));

    // Ensure invariants still hold after many insertions
    tree.insert(12, "j");
    tree.insert(13, "k");
    tree.insert(14, "l");
    tree.insert(11, "m");

    // All assertions passed if we reach this point
    println!("All tests passed.");
}
