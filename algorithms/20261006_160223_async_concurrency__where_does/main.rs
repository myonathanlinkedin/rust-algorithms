mod core;
use core::{block_on, YieldNow};

/// Example async computation that uses `YieldNow` to simulate cooperative yielding.
async fn compute_sum() -> i32 {
    let mut sum = 0;
    for i in 0..5 {
        sum += i;
        // Yield control back to the scheduler.
        YieldNow::new().await;
    }
    sum
}

fn main() {
    // Simple sanity check executed at runtime.
    let result = block_on(compute_sum());
    assert_eq!(result, 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::Executor;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_compute_sum() {
        let result = block_on(compute_sum());
        assert_eq!(result, 10);
    }

    #[test]
    fn test_multiple_tasks() {
        // Shared state to verify that tasks run concurrently.
        let shared = Arc::new(Mutex::new(Vec::new()));
        let exec = Executor::new();

        for id in 0..3 {
            let vec_clone = shared.clone();
            exec.spawn(async move {
                // Each task yields twice before pushing its id.
                YieldNow::new().await;
                YieldNow::new().await;
                vec_clone.lock().unwrap().push(id);
            });
        }

        exec.run();

        let mut result = shared.lock().unwrap().clone();
        result.sort();
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_yield_now_behavior() {
        // Ensure that `YieldNow` indeed yields once.
        let mut count = 0;
        let fut = async {
            count += 1;
            YieldNow::new().await;
            count += 1;
        };
        block_on(fut);
        assert_eq!(count, 2);
    }
}