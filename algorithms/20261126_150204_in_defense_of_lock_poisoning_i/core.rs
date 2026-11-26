use std::sync::{Mutex, MutexGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct PoisonError {
    pub msg: &'static str,

}

impl std::fmt::Display for PoisonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PoisonError: {}", self.msg)
    }
}
impl std::error::Error for PoisonError {}

pub struct PoisonableMutex<T> {
    pub inner: Mutex<T>,
    pub poisoned: AtomicBool,

}

impl<T> PoisonableMutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            inner: Mutex::new(data),
            poisoned: AtomicBool::new(false),
        }
    }

    /// Attempts to acquire the lock.
    /// Returns Err if the mutex is poisoned.
    pub fn lock(&self) -> Result<PoisonGuard<T>, PoisonError> {
        if self.poisoned.load(Ordering::SeqCst) {
            return Err(PoisonError { msg: "mutex is poisoned" });
        }
        match self.inner.lock() {
            Ok(guard) => Ok(PoisonGuard {
                mutex: self,
                guard,
            }),
            Err(_) => Err(PoisonError { msg: "failed to lock mutex" }),
        }
    }

    /// Returns true if the mutex has been poisoned.
    pub fn is_poisoned(&self) -> bool {
        self.poisoned.load(Ordering::SeqCst)
    }

    /// Clears the poisoned state, allowing further locking.
    pub fn clear_poison(&self) {
        self.poisoned.store(false, Ordering::SeqCst);
    }

    /// Consumes the mutex and returns the inner value.
    pub fn into_inner(self) -> T {
        // `into_inner` on std::sync::Mutex is stable since 1.63
        self.inner.into_inner().unwrap()
    }
}

pub struct PoisonGuard<'a, T> {
    pub mutex: &'a PoisonableMutex<T>,
    pub guard: MutexGuard<'a, T>,

}

impl<'a, T> Deref for PoisonGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a, T> DerefMut for PoisonGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

impl<'a, T> Drop for PoisonGuard<'a, T> {
    fn drop(&mut self) {
        // If the thread is unwinding (panic), mark the mutex as poisoned.
        if std::thread::panicking() {
            self.mutex.poisoned.store(true, Ordering::SeqCst);
        }
        // MutexGuard's Drop will automatically release the lock.
    }
}