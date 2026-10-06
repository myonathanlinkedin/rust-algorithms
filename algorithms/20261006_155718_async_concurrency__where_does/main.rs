mod core;
use core::{Executor, YieldNow};

use std::sync::{Arc, Mutex};

fn main() {
    // Simple demonstration that the executor runs two tasks.
    let exec = Executor::new();

    let result_a = Arc::new(Mutex::new(0usize));
    let result_b = Arc::new(Mutex::new(0usize));

    let ra = result_a.clone();
    exec.spawn(async move {
        YieldNow::new().await;
        *ra.lock().unwrap() = 1;
    });

    let rb = result_b.clone();
    exec.spawn(async move {
        YieldNow::new().await;
        YieldNow::new().await;
        *rb.lock().unwrap() = 2;
    });

    exec.run();

    assert_eq!(*result_a.lock().unwrap(), 1);
    assert_eq!(*result_b.lock().unwrap(), 2);
}

// ---------- Unit tests ----------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn single_task_completes() {
        let exec = Executor::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let c = counter.clone();

        exec.spawn(async move {
            YieldNow::new().await;
            c.fetch_add(1, Ordering::SeqCst);
        });

        exec.run();
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn multiple_tasks_interleave() {
        let exec = Executor::new();
        let order = Arc::new(Mutex::new(Vec::new()));

        let o1 = order.clone();
        exec.spawn(async move {
            o1.lock().unwrap().push(1);
            YieldNow::new().await;
            o1.lock().unwrap().push(3);
        });

        let o2 = order.clone();
        exec.spawn(async move {
            o2.lock().unwrap().push(2);
            YieldNow::new().await;
            o2.lock().unwrap().push(4);
        });

        exec.run();

        let recorded = order.lock().unwrap().clone();
        // Expected interleaving: 1,2,3,4 or 2,1,4,3 depending on scheduling order.
        // Both are valid; we just ensure all numbers appear.
        assert_eq!(recorded.len(), 4);
        for i in 1..=4 {
            assert!(recorded.contains(&i));
        }
    }

    #[test]
    fn task_wakes_multiple_times() {
        let exec = Executor::new();
        let flag = Arc::new(AtomicUsize::new(0));
        let f = flag.clone();

        exec.spawn(async move {
            // First poll: pending, wakes itself.
            YieldNow::new().await;
            // Second poll: pending again, wakes itself.
            YieldNow::new().await;
            // Final poll: ready.
            f.store(99, Ordering::SeqCst);
        });

        exec.run();
        assert_eq!(flag.load(Ordering::SeqCst), 99);
    }
}

// === END OF main.rs ===