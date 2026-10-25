use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

#[derive(Debug)]
pub struct BoundedQueue<T> {
    pub capacity: usize,
    pub inner: Mutex<Inner<T>>,
    pub not_empty: Condvar,
    pub not_full: Condvar,

}

#[derive(Debug)]
pub struct Inner<T> {
    pub queue: VecDeque<T>,

}

impl<T> BoundedQueue<T> {
    /// Creates a new bounded queue with the given capacity.
    /// Panics if `capacity` is zero.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");
        BoundedQueue {
            capacity,
            inner: Mutex::new(Inner {
                queue: VecDeque::with_capacity(capacity),
            }),
            not_empty: Condvar::new(),
            not_full: Condvar::new(),
        }
    }

    /// Inserts `item` into the queue, blocking if the queue is full.
    pub fn push(&self, item: T) {
        let mut guard = self.inner.lock().unwrap();
        while guard.queue.len() == self.capacity {
            guard = self.not_full.wait(guard).unwrap();
        }
        guard.queue.push_back(item);
        // Wake one thread waiting for items.
        self.not_empty.notify_one();
    }

    /// Removes and returns the front element of the queue, blocking if the queue is empty.
    pub fn pop(&self) -> T {
        let mut guard = self.inner.lock().unwrap();
        while guard.queue.is_empty() {
            guard = self.not_empty.wait(guard).unwrap();
        }
        let item = guard.queue.pop_front().unwrap();
        // Wake one thread waiting for space.
        self.not_full.notify_one();
        item
    }

    /// Returns the current number of elements in the queue.
    pub fn len(&self) -> usize {
        let guard = self.inner.lock().unwrap();
        guard.queue.len()
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if the queue is full.
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }
}