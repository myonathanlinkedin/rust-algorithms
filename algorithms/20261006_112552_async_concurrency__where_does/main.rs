mod core;
use core::{YieldNow, SimpleExecutor};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// An async function that increments a shared counter.
async fn async_increment(counter: Arc<AtomicUsize>) {
    counter.fetch_add(1, Ordering::SeqCst);
}

/// An async function that yields once before incrementing.
async fn async_yield_then_increment(counter: Arc<AtomicUsize>) {
    YieldNow::new().await;
    counter.fetch_add(1, Ordering::SeqCst);
}

fn main() {
    // Shared state for verification.
    let counter = Arc::new(AtomicUsize::new(0));

    // Create executor and spawn tasks.
    let mut exec = SimpleExecutor::new();
    exec.spawn(async_increment(counter.clone()));
    exec.spawn(async_yield_then_increment(counter.clone()));
    exec.spawn(async {
        // Nested async block using YieldNow.
        YieldNow::new().await;
    });
    exec.run();

    // Verify that both increments occurred.
    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn test_simple_executor() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut exec = SimpleExecutor::new();

        // Spawn multiple tasks.
        for _ in 0..5 {
            exec.spawn(async_increment(counter.clone()));
        }
        exec.run();

        assert_eq!(counter.load(Ordering::SeqCst), 5);
    }

    #[test]
    fn test_yield_behavior() {
        let order = Arc::new(Mutex::new(Vec::new()));
        let mut exec = SimpleExecutor::new();

        let order_clone = order.clone();
        exec.spawn(async move {
            order_clone.lock().unwrap().push(1);
            YieldNow::new().await;
            order_clone.lock().unwrap().push(3);
        });

        let order_clone = order.clone();
        exec.spawn(async move {
            order_clone.lock().unwrap().push(2);
        });

        exec.run();

        // Expected order: 1 (first task before yield), 2 (second task runs while first yielded), 3 (first task resumes)
        let result = order.lock().unwrap().clone();
        assert_eq!(result, vec![1, 2, 3]);
    }
}