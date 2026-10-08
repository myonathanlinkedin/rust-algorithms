use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[derive(Debug)]
pub struct AtomicCounter {
    pub value: AtomicUsize,

}

impl AtomicCounter {
    pub fn new(initial: usize) -> Self {
        Self {
            value: AtomicUsize::new(initial),
        }
    }

    /// Atomically increments the counter and returns the new value.
    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Returns the current value without modifying it.
    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    /// Performs a compare‑and‑set operation.
    /// Returns true if the value was equal to `current` and was replaced with `new`.
    pub fn compare_and_set(&self, current: usize, new: usize) -> bool {
        self.value
            .compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    }
}

// Simple spin‑lock implementation using an AtomicBool.
pub struct SpinLock<T> {
    pub lock: AtomicBool,
    pub data: UnsafeCell<T>,

}

// Safety: `SpinLock<T>` provides interior mutability with proper synchronization.
unsafe impl<T: Send> Sync for SpinLock<T> {}
unsafe impl<T: Send> Send for SpinLock<T> {}

pub struct SpinLockGuard<'a, T> {
    pub lock: &'a AtomicBool,
    pub data: &'a mut T,

}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Acquires the lock, spinning until it becomes available.
    pub fn lock(&self) -> SpinLockGuard<'_, T> {
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // busy‑wait
            std::hint::spin_loop();
        }
        // SAFETY: we have exclusive access because we hold the lock.
        let data = unsafe { &mut *self.data.get() };
        SpinLockGuard {
            lock: &self.lock,
            data,
        }
    }
}

impl<'a, T> std::ops::Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> std::ops::DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.store(false, Ordering::Release);
    }
}