mod core;
use core::{Executor, YieldNow};

use std::sync::{Arc, Mutex};

fn main() {
    // Simple demonstration: two tasks interleaving via YieldNow.
    let exec = Executor::new();
    let log = Arc::new(Mutex::new(Vec::new()));

    let log1 = Arc::clone(&log);
    exec.spawn(async move {
        log1.lock().unwrap().push(1);
        YieldNow::new().await;
        log1.lock().unwrap().push(3);
    });

    let log2 = Arc::clone(&log);
    exec.spawn(async move {
        log2.lock().unwrap().push(2);
        YieldNow::new().await;
        log2.lock().unwrap().push(4);
    });

    exec.run();

    let result = log.lock().unwrap().clone();
    // Expected interleaving: 1,2,3,4 (order may vary but each task yields once)
    assert_eq!(result, vec![1, 2, 3, 4]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_yield_now() {
        let mut count = 0usize;
        let fut = async {
            count += 1;
            YieldNow::new().await;
            count += 1;
        };
        let exec = Executor::new();
        exec.spawn(fut);
        exec.run();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_multiple_tasks_order() {
        let exec = Executor::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let out = Arc::new(Mutex::new(Vec::new()));

        for i in 0..3usize {
            let ctr = Arc::clone(&counter);
            let out_clone = Arc::clone(&out);
            exec.spawn(async move {
                let id = ctr.fetch_add(1, Ordering::SeqCst);
                out_clone.lock().unwrap().push(id * 2);
                YieldNow::new().await;
                out_clone.lock().unwrap().push(id * 2 + 1);
            });
        }

        exec.run();

        let mut result = out.lock().unwrap().clone();
        // Sort to verify all expected numbers are present.
        result.sort_unstable();
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_executor_completes_all() {
        let exec = Executor::new();
        let flag = Arc::new(AtomicUsize::new(0));

        for _ in 0..5 {
            let f = Arc::clone(&flag);
            exec.spawn(async move {
                f.fetch_add(1, Ordering::SeqCst);
                YieldNow::new().await;
                f.fetch_add(1, Ordering::SeqCst);
            });
        }

        exec.run();
        // Each task increments twice.
        assert_eq!(flag.load(Ordering::SeqCst), 10);
    }
}