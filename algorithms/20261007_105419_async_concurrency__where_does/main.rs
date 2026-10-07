mod core;
use core::{Executor, Task};

use std::sync::{Arc, Mutex};

/// Simple async function that adds two numbers after an await point.
async fn async_add(a: i32, b: i32) -> i32 {
    // Simulate an async boundary.
    async {}.await;
    a + b
}

/// Async task that records its result into a shared vector.
async fn record_sum(vec: Arc<Mutex<Vec<i32>>>, a: i32, b: i32) {
    let sum = async_add(a, b).await;
    let mut guard = vec.lock().unwrap();
    guard.push(sum);
}

/// Demonstrates spawning multiple tasks and running the executor.
fn demo() {
    let mut exec = Executor::new();
    let results = Arc::new(Mutex::new(Vec::new()));

    exec.spawn(record_sum(results.clone(), 1, 2));
    exec.spawn(record_sum(results.clone(), 10, 20));
    exec.spawn(async move {
        // Nested async block with a delay simulation.
        async {}.await;
    });

    exec.run();

    let guard = results.lock().unwrap();
    assert_eq!(guard.len(), 2);
    assert!(guard.contains(&3));
    assert!(guard.contains(&30));
}

fn main() {
    demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::Executor;

    #[test]
    fn test_basic_addition() {
        let mut exec = Executor::new();
        let out = Arc::new(Mutex::new(Vec::new()));
        exec.spawn(record_sum(out.clone(), 5, 7));
        exec.run();
        let guard = out.lock().unwrap();
        assert_eq!(guard.as_slice(), &[12]);
    }

    #[test]
    fn test_multiple_tasks() {
        let mut exec = Executor::new();
        let out = Arc::new(Mutex::new(Vec::new()));
        for i in 0..5 {
            exec.spawn(record_sum(out.clone(), i, i * 2));
        }
        exec.run();
        let mut guard = out.lock().unwrap();
        guard.sort();
        assert_eq!(guard, vec![0, 3, 6, 9, 12]);
    }

    #[test]
    fn test_nested_async() {
        async fn inner() -> i32 {
            async {}.await;
            42
        }

        async fn outer(vec: Arc<Mutex<Vec<i32>>>) {
            let v = inner().await;
            vec.lock().unwrap().push(v);
        }

        let mut exec = Executor::new();
        let out = Arc::new(Mutex::new(Vec::new()));
        exec.spawn(outer(out.clone()));
        exec.run();
        let guard = out.lock().unwrap();
        assert_eq!(guard.as_slice(), &[42]);
    }
}