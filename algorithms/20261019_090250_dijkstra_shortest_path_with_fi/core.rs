use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone)]
pub struct Graph {
    pub adj: Vec<Vec<(usize, u64)>>,

}

impl Graph {
    pub fn new(num_vertices: usize) -> Self {
        Self {
            adj: vec![Vec::new(); num_vertices],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: u64) {
        self.adj[from].push((to, weight));
    }

    pub fn add_undirected(&mut self, a: usize, b: usize, weight: u64) {
        self.add_edge(a, b, weight);
        self.add_edge(b, a, weight);
    }
}

#[derive(Debug)]
pub struct Node<K: Ord + Clone, V: Clone> {
    pub key: K,
    pub value: V,
    pub degree: usize,
    pub mark: bool,
    pub parent: Option<Weak<RefCell<Node<K, V>>>>,
    pub child: Option<Rc<RefCell<Node<K, V>>>>,
    pub left: Option<Rc<RefCell<Node<K, V>>>>,
    pub right: Option<Rc<RefCell<Node<K, V>>>>,

}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    fn new(key: K, value: V) -> Rc<RefCell<Self>> {
        let node = Rc::new(RefCell::new(Node {
            key,
            value,
            degree: 0,
            mark: false,
            parent: None,
            child: None,
            left: None,
            right: None,
        }));
        node.borrow_mut().left = Some(Rc::clone(&node));
        node.borrow_mut().right = Some(Rc::clone(&node));
        node
    }
}

pub struct FibHeap<K: Ord + Clone, V: Clone> {
    pub min: Option<Rc<RefCell<Node<K, V>>>>,
    pub n: usize,

}

impl<K: Ord + Clone, V: Clone> FibHeap<K, V> {
    pub fn new() -> Self {
        Self { min: None, n: 0 }
    }

    pub fn insert(&mut self, key: K, value: V) -> Rc<RefCell<Node<K, V>>> {
        let node = Node::new(key, value);
        if let Some(ref min) = self.min {
            self.splice_into_root_list(&node, min);
            if node.borrow().key < min.borrow().key {
                self.min = Some(Rc::clone(&node));
            }
        } else {
            self.min = Some(Rc::clone(&node));
        }
        self.n += 1;
        node
    }

    pub fn extract_min(&mut self) -> Option<(K, V)> {
        let z = self.min.take()?;
        // move children to root list
        if let Some(child) = z.borrow().child.clone() {
            let mut children = Vec::new();
            let mut c = child.clone();
            loop {
                children.push(c.clone());
                let nxt = c.borrow().right.as_ref().unwrap().clone();
                if Rc::ptr_eq(&nxt, &child) {
                    break;
                }
                c = nxt;
            }
            for child in children {
                child.borrow_mut().parent = None;
                self.add_to_root_list(child);
            }
        }
        // remove z from root list
        self.remove_from_root_list(&z);
        self.n -= 1;
        if self.n == 0 {
            self.min = None;
        } else {
            self.min = z.borrow().right.clone();
            self.consolidate();
        }
        // unwrap node without extra refs
        let node = Rc::try_unwrap(z).ok().expect("unique reference").into_inner();
        Some((node.key, node.value))
    }

    pub fn decrease_key(&mut self, node: Rc<RefCell<Node<K, V>>>, new_key: K) {
        if new_key > node.borrow().key {
            panic!("new key is greater than current key");
        }
        node.borrow_mut().key = new_key.clone();
        if let Some(parent_weak) = node.borrow().parent.clone() {
            if let Some(parent) = parent_weak.upgrade() {
                if node.borrow().key < parent.borrow().key {
                    self.cut(node.clone(), parent.clone());
                    self.cascading_cut(parent);
                }
            }
        }
        if self.min.is_none() || node.borrow().key < self.min.as_ref().unwrap().borrow().key {
            self.min = Some(node);
        }
    }

    // ----- internal helpers -----
    fn splice_into_root_list(&self, node: &Rc<RefCell<Node<K, V>>>, min: &Rc<RefCell<Node<K, V>>>) {
        let min_left = min.borrow().left.as_ref().unwrap().clone();
        node.borrow_mut().right = Some(Rc::clone(min));
        node.borrow_mut().left = Some(min_left.clone());
        min_left.borrow_mut().right = Some(Rc::clone(node));
        min.borrow_mut().left = Some(Rc::clone(node));
    }

    fn add_to_root_list(&mut self, node: Rc<RefCell<Node<K, V>>>) {
        if let Some(ref min) = self.min {
            self.splice_into_root_list(&node, min);
            if node.borrow().key < min.borrow().key {
                self.min = Some(node);
            }
        } else {
            node.borrow_mut().left = Some(Rc::clone(&node));
            node.borrow_mut().right = Some(Rc::clone(&node));
            self.min = Some(node);
        }
    }

    fn remove_from_root_list(&self, node: &Rc<RefCell<Node<K, V>>>) {
        let left = node.borrow().left.as_ref().unwrap().clone();
        let right = node.borrow().right.as_ref().unwrap().clone();
        left.borrow_mut().right = Some(right.clone());
        right.borrow_mut().left = Some(left);
    }

    fn consolidate(&mut self) {
        let max_degree = (self.n as f64).log2().ceil() as usize + 2;
        let mut a: Vec<Option<Rc<RefCell<Node<K, V>>>>> = vec![None; max_degree];
        // collect roots
        let mut roots = Vec::new();
        if let Some(min) = self.min.clone() {
            let mut x = min.clone();
            loop {
                roots.push(x.clone());
                let nxt = x.borrow().right.as_ref().unwrap().clone();
                if Rc::ptr_eq(&nxt, &min) {
                    break;
                }
                x = nxt;
            }
        }
        for w in roots {
            let mut x = w;
            let mut d = x.borrow().degree;
            while let Some(y) = a[d].take() {
                if x.borrow().key > y.borrow().key {
                    std::mem::swap(&mut x, &mut y.clone());
                }
                self.heap_link(y, x.clone());
                d += 1;
            }
            a[d] = Some(x);
        }
        self.min = None;
        for node_opt in a.into_iter() {
            if let Some(node) = node_opt {
                if self.min.is_none() {
                    node.borrow_mut().left = Some(Rc::clone(&node));
                    node.borrow_mut().right = Some(Rc::clone(&node));
                    self.min = Some(node);
                } else {
                    self.add_to_root_list(node.clone());
                    if node.borrow().key < self.min.as_ref().unwrap().borrow().key {
                        self.min = Some(node);
                    }
                }
            }
        }
    }

    fn heap_link(&mut self, y: Rc<RefCell<Node<K, V>>>, x: Rc<RefCell<Node<K, V>>>) {
        self.remove_from_root_list(&y);
        y.borrow_mut().parent = Some(Rc::downgrade(&x));
        y.borrow_mut().mark = false;
        if let Some(child) = x.borrow().child.clone() {
            // insert y into child's sibling list
            let child_left = child.borrow().left.as_ref().unwrap().clone();
            y.borrow_mut().right = Some(child.clone());
            y.borrow_mut().left = Some(child_left.clone());
            child_left.borrow_mut().right = Some(y.clone());
            child.borrow_mut().left = Some(y.clone());
        } else {
            x.borrow_mut().child = Some(y.clone());
            y.borrow_mut().left = Some(y.clone());
            y.borrow_mut().right = Some(y.clone());
        }
        x.borrow_mut().degree += 1;
    }

    fn cut(&mut self, x: Rc<RefCell<Node<K, V>>>, y: Rc<RefCell<Node<K, V>>>) {
        // detach x from y's child list
        if let Some(child) = y.borrow().child.clone() {
            if Rc::ptr_eq(&x, &child) {
                if x.borrow().right.as_ref().unwrap().as_ptr() == x.as_ptr() {
                    y.borrow_mut().child = None;
                } else {
                    y.borrow_mut().child = Some(x.borrow().right.as_ref().unwrap().clone());
                }
            }
        }
        // adjust sibling links
        let left = x.borrow().left.as_ref().unwrap().clone();
        let right = x.borrow().right.as_ref().unwrap().clone();
        left.borrow_mut().right = Some(right.clone());
        right.borrow_mut().left = Some(left);
        y.borrow_mut().degree -= 1;
        // add x to root list
        x.borrow_mut().parent = None;
        x.borrow_mut().mark = false;
        self.add_to_root_list(x);
    }

    fn cascading_cut(&mut self, y: Rc<RefCell<Node<K, V>>>) {
        if let Some(parent_weak) = y.borrow().parent.clone() {
            if let Some(z) = parent_weak.upgrade() {
                if !y.borrow().mark {
                    y.borrow_mut().mark = true;
                } else {
                    self.cut(y.clone(), z.clone());
                    self.cascading_cut(z);
                }
            }
        }
    }
}

pub fn dijkstra(graph: &Graph, source: usize) -> Vec<u64> {
    let n = graph.adj.len();
    let mut dist = vec![u64::MAX; n];
    dist[source] = 0;
    let mut heap = FibHeap::new();
    let mut handles: Vec<Option<Rc<RefCell<Node<u64, usize>>>>> = vec![None; n];
    handles[source] = Some(heap.insert(0, source));
    while let Some((d, u)) = heap.extract_min() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &graph.adj[u] {
            let alt = d.saturating_add(w);
            if alt < dist[v] {
                dist[v] = alt;
                if let Some(handle) = handles[v].clone() {
                    heap.decrease_key(handle, alt);
                } else {
                    handles[v] = Some(heap.insert(alt, v));
                }
            }
        }
    }
    dist
}