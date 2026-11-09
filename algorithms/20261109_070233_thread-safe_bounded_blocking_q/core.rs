use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
pub struct BoundedBlockingQueue<T> {
    pub capacity: usize,
    pub inner: Mutex<Inner<T>>,
    pub not_full: Condvar,
    pub not_empty: Condvar,

}

#[derive(Debug)]
pub struct Inner<T> {
    pub queue: VecDeque<T>,

}

impl<T> BoundedBlockingQueue<T> {
    /// Creates a new queue with the given positive capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be > 0");
        BoundedBlockingQueue {
            capacity,
            inner: Mutex::new(Inner {
                queue: VecDeque::with_capacity(capacity),
            }),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }

    /// Inserts `item` into the queue, blocking if the queue is full.
    pub fn push(&self, item: T) {
        let mut guard = self.inner.lock().unwrap();
        while guard.queue.len() == self.capacity {
            guard = self.not_full.wait(guard).unwrap();
        }
        guard.queue.push_back(item);
        self.not_empty.notify_one();
    }

    /// Removes and returns the front element, blocking if the queue is empty.
    pub fn pop(&self) -> T {
        let mut guard = self.inner.lock().unwrap();
        while guard.queue.is_empty() {
            guard = self.not_empty.wait(guard).unwrap();
        }
        let item = guard.queue.pop_front().unwrap();
        self.not_full.notify_one();
        item
    }

    /// Attempts to insert `item` without blocking.
    /// Returns `Ok(())` on success, or `Err(item)` if the queue is full.
    pub fn try_push(&self, item: T) -> Result<(), T> {
        let mut guard = self.inner.lock().unwrap();
        if guard.queue.len() == self.capacity {
            Err(item)
        } else {
            guard.queue.push_back(item);
            self.not_empty.notify_one();
            Ok(())
        }
    }

    /// Attempts to remove the front element without blocking.
    /// Returns `Some(item)` if successful, or `None` if the queue is empty.
    pub fn try_pop(&self) -> Option<T> {
        let mut guard = self.inner.lock().unwrap();
        let opt = guard.queue.pop_front();
        if opt.is_some() {
            self.not_full.notify_one();
        }
        opt
    }

    /// Returns the current number of elements.
    pub fn len(&self) -> usize {
        let guard = self.inner.lock().unwrap();
        guard.queue.len()
    }

    /// Returns `true` if the queue contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the queue has reached its capacity.
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }
}