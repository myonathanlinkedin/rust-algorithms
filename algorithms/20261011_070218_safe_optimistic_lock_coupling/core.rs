use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub next: Mutex<Option<Arc<Node>>>,

}

impl Node {
    pub fn new(value: i32, next: Option<Arc<Node>>) -> Arc<Node> {
        Arc::new(Node {
            value,
            next: Mutex::new(next),
        })
    }
}

#[derive(Debug)]
pub struct OptimisticList {
    pub head: Arc<Node>, // sentinel node with value i32::MIN

}

impl OptimisticList {
    pub fn new() -> Self {
        OptimisticList {
            head: Node::new(i32::MIN, None),
        }
    }

    // Helper: locate predecessor and (optional) current node without holding locks
    fn find(&self, key: i32) -> (Arc<Node>, Option<Arc<Node>>) {
        let mut pred = self.head.clone();
        loop {
            let next_opt = {
                let guard = pred.next.lock().unwrap();
                guard.clone()
            };
            match next_opt {
                Some(ref curr) if curr.value < key => {
                    pred = curr.clone();
                }
                _ => {
                    return (pred, next_opt);
                }
            }
        }
    }

    // Helper: validate that pred.next still points to expected (or None)
    fn validate(pred: &Arc<Node>, expected: &Option<Arc<Node>>) -> bool {
        let guard = pred.next.lock().unwrap();
        match (&*guard, expected) {
            (Some(ref actual), Some(ref exp)) => Arc::ptr_eq(actual, exp),
            (None, None) => true,
            _ => false,
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        loop {
            let (pred, curr_opt) = self.find(key);
            // Acquire lock on predecessor to validate
            if !Self::validate(&pred, &curr_opt) {
                continue; // retry
            }
            if let Some(ref curr) = curr_opt {
                if curr.value == key {
                    return true;
                }
            }
            return false;
        }
    }

    pub fn insert(&self, key: i32) -> bool {
        loop {
            let (pred, curr_opt) = self.find(key);
            // Lock predecessor
            let mut pred_next = pred.next.lock().unwrap();
            // Validate adjacency
            if !Self::validate(&pred, &curr_opt) {
                continue; // retry
            }
            // If key already present, abort
            if let Some(ref curr) = curr_opt {
                if curr.value == key {
                    return false;
                }
            }
            // Create new node pointing to current
            let new_node = Node::new(key, curr_opt.clone());
            *pred_next = Some(new_node);
            return true;
        }
    }

    pub fn remove(&self, key: i32) -> bool {
        loop {
            let (pred, curr_opt) = self.find(key);
            // Lock predecessor
            let mut pred_next = pred.next.lock().unwrap();
            // Validate adjacency
            if !Self::validate(&pred, &curr_opt) {
                continue; // retry
            }
            // If not found, abort
            let curr = match curr_opt {
                Some(ref c) => c.clone(),
                None => return false,
            };
            if curr.value != key {
                return false;
            }
            // Lock current node to splice it out
            let mut curr_next = curr.next.lock().unwrap();
            *pred_next = curr_next.clone();
            return true;
        }
    }

    // Utility for testing: collect all values into a sorted Vec
    pub fn to_vec(&self) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current_opt = {
            let guard = self.head.next.lock().unwrap();
            guard.clone()
        };
        while let Some(node) = current_opt {
            result.push(node.value);
            current_opt = {
                let guard = node.next.lock().unwrap();
                guard.clone()
            };
        }
        result
    }
}