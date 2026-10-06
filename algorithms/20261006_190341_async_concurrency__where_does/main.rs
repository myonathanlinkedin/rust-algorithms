mod core;

use core::{async_increment, Executor};
use std::sync::{Arc, Mutex};

fn main() {
    let mut exec = Executor::new();
    let counter = Arc::new(Mutex::new(0u32));

    for _ in 0..5 {
        exec.spawn(async_increment(counter.clone()));
    }

    exec.run();

    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 5);
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::YieldNow;
    use std::future::Future;
    use std::task::{Context, Poll};

    #[test]
    fn test_yield_now() {
        let mut fut = Box::pin(YieldNow::new());
        let waker = core::dummy_waker();
        let mut cx = Context::from_waker(&waker);

        // First poll should be Pending.
        match fut.as_mut().poll(&mut cx) {
            Poll::Pending => {}
            _ => panic!("Expected Pending on first poll"),
        }

        // Second poll should be Ready.
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(()) => {}
            _ => panic!("Expected Ready on second poll"),
        }
    }

    #[test]
    fn test_executor_multiple_tasks() {
        let mut exec = Executor::new();
        let counter = Arc::new(Mutex::new(0u32));

        for _ in 0..10 {
            exec.spawn(async_increment(counter.clone()));
        }

        exec.run();

        let result = *counter.lock().unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn test_executor_no_tasks() {
        let mut exec = Executor::new();
        exec.run(); // Should complete without panicking.
    }

    #[test]
    fn test_executor_task_completes_immediately() {
        let mut exec = Executor::new();
        let flag = Arc::new(Mutex::new(false));

        let flag_clone = flag.clone();
        exec.spawn(async move {
            let mut guard = flag_clone.lock().unwrap();
            *guard = true;
        });

        exec.run();

        assert!(*flag.lock().unwrap());
    }
}