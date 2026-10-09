use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};

/// A thread‑safe bounded blocking queue.
///
/// The queue blocks on `push` when it is full and on `pop` when it is empty.
/// Internally it uses a `Mutex` to protect a `VecDeque` and two `Condvar`s
/// to signal state changes.
///
/// # Guarantees
/// * **Safety** – all public methods are safe and free of data races.
/// * **Liveness** – a waiting thread is awakened when the condition it waits
///   for becomes true (queue not full / not empty).
/// * **Boundedness** – the number of stored elements never exceeds `capacity`.
#[derive(Clone, Debug)]
pub struct BoundedQueue<T> {
    pub inner: Arc<Inner<T>>,

}

#[derive(Debug)]
pub struct Inner<T> {
    // Protects the queue and the current size.
    pub mutex: Mutex<State<T>>,
    // Signaled when an element is removed (queue becomes not full).
    pub not_full: Condvar,
    // Signaled when an element is inserted (queue becomes not empty).
    pub not_empty: Condvar,
    pub capacity: usize,

}

#[derive(Debug)]
pub struct State<T> {
    pub buffer: VecDeque<T>,

}

impl<T> BoundedQueue<T> {
    /// Creates a new bounded queue with the given `capacity`.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is zero because a zero‑capacity queue cannot make
    /// progress.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be greater than zero");
        let inner = Inner {
            mutex: Mutex::new(State {
                buffer: VecDeque::with_capacity(capacity),
            }),
            not_full: Condvar::new(),
            not_empty: Condvar::new(),
            capacity,
        };
        BoundedQueue {
            inner: Arc::new(inner),
        }
    }

    /// Inserts `item` into the queue, blocking the current thread if the queue
    /// is full until space becomes available.
    pub fn push(&self, item: T) {
        // Fast path: try to acquire lock and push without waiting.
        let mut guard = self.inner.mutex.lock().unwrap();
        while guard.buffer.len() == self.inner.capacity {
            // Queue is full; wait until a consumer removes an element.
            guard = self.inner.not_full.wait(guard).unwrap();
        }
        guard.buffer.push_back(item);
        // Wake up one waiting consumer, if any.
        self.inner.not_empty.notify_one();
        // `guard` is dropped here, releasing the lock.
    }

    /// Removes and returns an element from the queue, blocking the current
    /// thread if the queue is empty until an element becomes available.
    pub fn pop(&self) -> T {
        let mut guard = self.inner.mutex.lock().unwrap();
        while guard.buffer.is_empty() {
            // Queue is empty; wait until a producer inserts an element.
            guard = self.inner.not_empty.wait(guard).unwrap();
        }
        let item = guard
            .buffer
            .pop_front()
            .expect("buffer was non‑empty after condition check");
        // Wake up one waiting producer, if any.
        self.inner.not_full.notify_one();
        item
    }

    /// Returns the current number of elements in the queue.
    ///
    /// This method acquires the lock briefly; the returned value is only a
    /// snapshot and may become stale immediately after the call.
    pub fn len(&self) -> usize {
        let guard = self.inner.mutex.lock().unwrap();
        guard.buffer.len()
    }

    /// Returns the configured capacity of the queue.
    pub fn capacity(&self) -> usize {
        self.inner.capacity
    }
}