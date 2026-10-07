use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

mod core;
use core::{Executor, Task};

/// A future that yields once before completing.
struct YieldOnce {
    yielded: bool,
}

impl YieldOnce {
    fn new() -> Self {
        YieldOnce { yielded: false }
    }
}

impl Future for YieldOnce {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(42)
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}

fn main() {
    // Simple demonstration.
    let mut exec = Executor::new();
    let shared = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

    // Spawn three async tasks that push identifiers.
    for id in 1..=3u32 {
        let vec_clone = shared.clone();
        exec.spawn(async move {
            vec_clone.lock().unwrap().push(id);
        });
    }

    // Spawn a task using YieldOnce to test pending handling.
    exec.spawn(async move {
        let val = YieldOnce::new().await;
        assert_eq!(val, 42);
    });

    exec.run();

    let result = shared.lock().unwrap().clone();
    assert_eq!(result, vec![1, 2, 3]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_executor_basic() {
        let mut exec = Executor::new();
        let out = Arc::new(Mutex::new(Vec::new()));

        exec.spawn({
            let out = out.clone();
            async move {
                out.lock().unwrap().push(10);
            }
        });

        exec.spawn({
            let out = out.clone();
            async move {
                out.lock().unwrap().push(20);
            }
        });

        exec.run();

        let vec = out.lock().unwrap().clone();
        assert_eq!(vec, vec![10, 20]);
    }

    #[test]
    fn test_yield_once_future() {
        let mut exec = Executor::new();
        let result = Arc::new(Mutex::new(0u32));

        exec.spawn({
            let result = result.clone();
            async move {
                let val = YieldOnce::new().await;
                *result.lock().unwrap() = val;
            }
        });

        exec.run();

        assert_eq!(*result.lock().unwrap(), 42);
    }

    #[test]
    fn test_multiple_pending_tasks() {
        let mut exec = Executor::new();
        let counter = Arc::new(Mutex::new(0usize));

        // Create three tasks that each yield once before incrementing.
        for _ in 0..3 {
            let ctr = counter.clone();
            exec.spawn(async move {
                YieldOnce::new().await;
                let mut lock = ctr.lock().unwrap();
                *lock += 1;
            });
        }

        exec.run();

        assert_eq!(*counter.lock().unwrap(), 3);
    }
}