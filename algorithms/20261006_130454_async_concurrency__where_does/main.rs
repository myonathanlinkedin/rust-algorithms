mod types;
mod engine;

use std::sync::{Arc, Mutex};

use engine::Executor;

/// Simple async task that records a value into a shared vector.
async fn record_value(shared: Arc<Mutex<Vec<i32>>>, val: i32) {
    // Simulate some work; in a real async context you might await I/O.
    shared.lock().unwrap().push(val);
}

fn main() {
    // Shared state for verification.
    let results = Arc::new(Mutex::new(Vec::<i32>::new()));

    // Create the executor.
    let exec = Executor::new();

    // Spawn several tasks.
    exec.spawn(record_value(results.clone(), 10));
    exec.spawn(record_value(results.clone(), 20));
    exec.spawn(record_value(results.clone(), 30));

    // Run the executor to completion.
    exec.run();

    // Verify that all values were recorded.
    let mut collected = results.lock().unwrap().clone();
    collected.sort_unstable(); // Order is not guaranteed.
    assert_eq!(collected, vec![10, 20, 30]);

    // Additional edge case: spawning after the queue is empty.
    exec.spawn(record_value(results.clone(), 40));
    exec.run();
    let mut final_vec = results.lock().unwrap().clone();
    final_vec.sort_unstable();
    assert_eq!(final_vec, vec![10, 20, 30, 40]);

    // Demonstrate that the executor can be reused.
    exec.spawn(async {
        // No-op async block.
    });
    exec.run(); // Should complete without panics.

    // All assertions passed.
}