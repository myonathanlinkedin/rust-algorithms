use std::sync::Arc;

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

/// A thread‑safe bounded blocking queue.
///
/// The queue blocks on `push` when it is full and on `pop` when it is empty.
/// It uses a `Mutex` to protect the internal `VecDeque` and two `Condvar`s
/// to signal state changes.
///
/// # Type parameters
///
/// * `T` – The element type. Must be `Send` because the queue can be shared
///   across threads via `Arc`.
#[derive(Debug)]
pub struct BoundedQueue<T>
where
    T: Send,
{
    capacity: usize,
    inner: Mutex<Inner<T>>,
    not_full: Condvar,
    not_empty: Condvar,
}

#[derive(Debug)]
pub struct Inner<T> {
    pub queue: VecDeque<T>,

}

impl<T> BoundedQueue<T>
where
    T: Send,
{
    /// Creates a new bounded queue with the given capacity.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is zero.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");
        BoundedQueue {
            capacity,
            inner: Mutex::new(Inner {
                queue: VecDeque::with_capacity(capacity),
            }),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
        }
    }

    /// Pushes an element into the queue, blocking if the queue is full.
    ///
    /// This method returns only after the element has been inserted.
    pub fn push(&self, item: T) {
        let mut guard = self.inner.lock().unwrap();
        while guard.queue.len() == self.capacity {
            guard = self.not_full.wait(guard).unwrap();
        }
        guard.queue.push_back(item);
        // Wake up one consumer that might be waiting.
        self.not_empty.notify_one();
    }

    /// Pops an element from the queue, blocking if the queue is empty.
    ///
    /// Returns the removed element.
    pub fn pop(&self) -> T {
        let mut guard = self.inner.lock().unwrap();
        while guard.queue.is_empty() {
            guard = self.not_empty.wait(guard).unwrap();
        }
        let item = guard.queue.pop_front().unwrap();
        // Wake up one producer that might be waiting.
        self.not_full.notify_one();
        item
    }

    /// Attempts to push an element without blocking.
    ///
    /// Returns `Ok(())` if the element was inserted, or `Err(item)` if the
    /// queue was full.
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

    /// Attempts to pop an element without blocking.
    ///
    /// Returns `Some(item)` if an element was available, or `None` if the
    /// queue was empty.
    pub fn try_pop(&self) -> Option<T> {
        let mut guard = self.inner.lock().unwrap();
        let result = guard.queue.pop_front();
        if result.is_some() {
            self.not_full.notify_one();
        }
        result
    }

    /// Returns the current number of elements in the queue.
    ///
    /// This is a non‑blocking snapshot; the value may change immediately
    /// after the call returns.
    pub fn len(&self) -> usize {
        let guard = self.inner.lock().unwrap();
        guard.queue.len()
    }

    /// Returns `true` if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the queue is full.
    pub fn is_full(&self) -> bool {
        self.len() == self.capacity
    }
}