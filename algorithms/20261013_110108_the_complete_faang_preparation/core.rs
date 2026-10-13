use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node {
    pub key: i32,
    pub value: i32,
    pub prev: Option<Weak<RefCell<Node>>>,
    pub next: Option<Rc<RefCell<Node>>>,

}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]
pub struct LRUCache {
    pub capacity: usize,
    pub map: HashMap<i32, Rc<RefCell<Node>>>,
    pub head: Rc<RefCell<Node>>, // dummy head
    pub tail: Rc<RefCell<Node>>, // dummy tail

}

impl LRUCache {
    pub fn new(capacity: usize) -> Self {
        // Create dummy head and tail nodes
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);
        {
            let mut head_borrow = head.borrow_mut();
            head_borrow.next = Some(Rc::clone(&tail));
        }
        {
            let mut tail_borrow = tail.borrow_mut();
            tail_borrow.prev = Some(Rc::downgrade(&head));
        }
        LRUCache {
            capacity,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node_rc) = self.map.get(&key) {
            let value = node_rc.borrow().value;
            self.move_to_front(Rc::clone(node_rc));
            value
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node_rc) = self.map.get(&key) {
            // Update existing node and move to front
            node_rc.borrow_mut().value = value;
            self.move_to_front(Rc::clone(node_rc));
        } else {
            // Insert new node
            if self.map.len() == self.capacity {
                self.evict_lru();
            }
            let new_node = Node::new(key, value);
            self.add_to_front(Rc::clone(&new_node));
            self.map.insert(key, new_node);
        }
    }

    fn move_to_front(&mut self, node: Rc<RefCell<Node>>) {
        self.remove_node(Rc::clone(&node));
        self.add_to_front(node);
    }

    fn add_to_front(&mut self, node: Rc<RefCell<Node>>) {
        // Insert node right after head
        let mut node_borrow = node.borrow_mut();
        node_borrow.prev = Some(Rc::downgrade(&self.head));
        node_borrow.next = self.head.borrow().next.clone();

        if let Some(next_rc) = &node_borrow.next {
            next_rc.borrow_mut().prev = Some(Rc::downgrade(&node));
        }

        self.head.borrow_mut().next = Some(Rc::clone(&node));
    }

    fn remove_node(&mut self, node: Rc<RefCell<Node>>) {
        let prev_opt = node.borrow().prev.clone();
        let next_opt = node.borrow().next.clone();

        if let (Some(prev_weak), Some(next_rc)) = (prev_opt, next_opt) {
            if let Some(prev_rc) = prev_weak.upgrade() {
                prev_rc.borrow_mut().next = Some(Rc::clone(&next_rc));
                next_rc.borrow_mut().prev = Some(Rc::downgrade(&prev_rc));
            }
        }
    }

    fn evict_lru(&mut self) {
        // The LRU node is just before tail
        let lru_opt = self.tail.borrow().prev.clone().and_then(|w| w.upgrade());
        if let Some(lru_rc) = lru_opt {
            let key = lru_rc.borrow().key;
            self.remove_node(Rc::clone(&lru_rc));
            self.map.remove(&key);
        }
    }
}