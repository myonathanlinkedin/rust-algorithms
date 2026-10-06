use std::task::{Context, Poll};
mod core;
use core::{Executor, YieldNow};

use std::future::Future;
use std::sync::{Arc, Mutex};

/// An async task that increments a shared counter ten times,
/// yielding after each increment to demonstrate cooperative scheduling.
async fn async_increment(counter: Arc<Mutex<u32>>, _id: u32) {
    for _ in 0..10 {
        {
            let mut guard = counter.lock().unwrap();
            *guard += 1;
        }
        // Yield control back to the executor.
        YieldNow::new().await;
    }
}

fn main() {
    // Shared state for verification.
    let counter = Arc::new(Mutex::new(0u32));
    let exec = Executor::new();

    // Spawn five independent increment tasks.
    for i in 0..5 {
        let c = counter.clone();
        exec.spawn(async_increment(c, i));
    }

    // Run the scheduler to completion.
    exec.run();

    // Verify that the counter reflects all increments.
    assert_eq!(*counter.lock().unwrap(), 5 * 10);
}

// Unit tests exercising the executor and the YieldNow future.
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_yield_now() {
        let mut fut = Box::pin(YieldNow::new());
        let waker = core::dummy_waker();
        let mut ctx = std::task::Context::from_waker(&waker);

        // First poll should be pending.
        assert_eq!(fut.as_mut().poll(&mut ctx), std::task::Poll::Pending);
        // Second poll should be ready.
        assert_eq!(fut.as_mut().poll(&mut ctx), std::task::Poll::Ready(()));
    }

    #[test]
    fn test_executor_multiple_tasks() {
        let counter = Arc::new(Mutex::new(0u32));
        let exec = Executor::new();

        // Spawn three tasks, each incrementing 7 times.
        for i in 0..3 {
            let c = counter.clone();
            exec.spawn(async move {
                for _ in 0..7 {
                    {
                        let mut guard = c.lock().unwrap();
                        *guard += 1;
                    }
                    YieldNow::new().await;
                }
            });
        }

        exec.run();
        assert_eq!(*counter.lock().unwrap(), 3 * 7);
    }

    #[test]
    fn test_executor_no_tasks() {
        let exec = Executor::new();
        // Should complete instantly without panicking.
        exec.run();
    }

    #[test]
    fn test_executor_performance() {
        // Simple benchmark to ensure the executor makes progress.
        let counter = Arc::new(Mutex::new(0u32));
        let exec = Executor::new();

        for _ in 0..100 {
            let c = counter.clone();
            exec.spawn(async move {
                // Minimal work.
                YieldNow::new().await;
                let mut guard = c.lock().unwrap();
                *guard += 1;
            });
        }

        let start = Instant::now();
        exec.run();
        let duration = start.elapsed();

        assert_eq!(*counter.lock().unwrap(), 100);
        // The executor should finish quickly; allow generous bound.
        assert!(duration.as_millis() < 500);
    }
}