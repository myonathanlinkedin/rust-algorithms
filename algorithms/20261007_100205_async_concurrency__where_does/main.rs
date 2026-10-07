use std::pin::Pin;
use std::task::{Context, Poll};

mod core;
use core::{Executor, YieldNow, dummy_waker};

use std::sync::{Arc, Mutex};

/// Example async task that records its progress into a shared vector.
async fn record_task(id: usize, log: Arc<Mutex<Vec<usize>>>) {
    // First step.
    log.lock().unwrap().push(id);
    // Yield once.
    YieldNow::default().await;
    // Second step.
    log.lock().unwrap().push(id + 100);
}

fn main() {
    // Shared log for verification.
    let log = Arc::new(Mutex::new(Vec::new()));

    // Build executor and schedule two tasks.
    let mut exec = Executor::new();
    exec.spawn(record_task(1, Arc::clone(&log)));
    exec.spawn(record_task(2, Arc::clone(&log)));

    // Run the scheduler.
    exec.run();

    // Expected interleaved order: 1, 2, 101, 102.
    let result = log.lock().unwrap().clone();
    assert_eq!(result, vec![1usize, 2, 101, 102]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_executor_interleaving() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let mut exec = Executor::new();
        exec.spawn(record_task(10, Arc::clone(&log)));
        exec.spawn(record_task(20, Arc::clone(&log)));
        exec.run();
        let result = log.lock().unwrap().clone();
        assert_eq!(result, vec![10usize, 20, 110, 120]);
    }

    #[test]
    fn test_yield_now_behavior() {
        let mut fut = YieldNow::default();
        let waker = dummy_waker();
        let mut cx = std::task::Context::from_waker(&waker);
        // First poll should be pending.
        assert_eq!(Pin::new(&mut fut).poll(&mut cx), std::task::Poll::Pending);
        // Second poll should be ready.
        assert_eq!(Pin::new(&mut fut).poll(&mut cx), std::task::Poll::Ready(()));
    }
}