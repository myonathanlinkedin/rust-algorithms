mod core;
use core::SimpleExecutor;
use std::sync::{Arc, Mutex};

/// Simple async addition function.
async fn async_add(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    // Demonstrate the executor with a shared counter.
    let executor = SimpleExecutor::new();
    let counter = Arc::new(Mutex::new(0usize));

    // Spawn a task that increments the counter five times.
    let c = counter.clone();
    executor.spawn(async move {
        for _ in 0..5 {
            let mut lock = c.lock().unwrap();
            *lock += 1;
        }
    });

    // Spawn another task that uses async_add and stores the result.
    let results = Arc::new(Mutex::new(Vec::new()));
    let r = results.clone();
    executor.spawn(async move {
        let sum = async_add(7, 8).await;
        r.lock().unwrap().push(sum);
    });

    executor.run();

    // Assertions to verify correct behavior.
    assert_eq!(*counter.lock().unwrap(), 5);
    let res = results.lock().unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0], 15);
}

// ---------- Unit tests ----------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::Mutex;

    #[test]
    fn test_multiple_tasks() {
        let executor = SimpleExecutor::new();
        let results = Arc::new(Mutex::new(Vec::new()));

        // Task 1
        let r1 = results.clone();
        executor.spawn(async move {
            let sum = async_add(2, 3).await;
            r1.lock().unwrap().push(sum);
        });

        // Task 2
        let r2 = results.clone();
        executor.spawn(async move {
            let sum = async_add(10, 5).await;
            r2.lock().unwrap().push(sum);
        });

        executor.run();

        let mut guard = results.lock().unwrap();
        guard.sort(); // order is nondeterministic
        assert_eq!(*guard, vec![5, 15]);
    }

    #[test]
    fn test_sequential_increment() {
        let executor = SimpleExecutor::new();
        let counter = Arc::new(Mutex::new(0usize));

        for i in 0..3 {
            let c = counter.clone();
            executor.spawn(async move {
                let mut lock = c.lock().unwrap();
                *lock += i + 1;
            });
        }

        executor.run();
        assert_eq!(*counter.lock().unwrap(), 6); // 1 + 2 + 3
    }
}