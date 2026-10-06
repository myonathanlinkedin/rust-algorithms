mod core;
use core::Executor;
use std::sync::{Arc, Mutex};
use std::future;
use std::future::Ready;

/// Simple async function that increments a shared counter after awaiting a ready future.
async fn increment(counter: Arc<Mutex<u32>>, amount: u32) {
    // Simulate an async pause.
    future::ready(()).await;
    let mut lock = counter.lock().unwrap();
    *lock += amount;
}

/// Async function that spawns two child tasks and waits for them using `future::ready`.
async fn parent_task(counter: Arc<Mutex<u32>>) {
    // Child 1
    increment(counter.clone(), 2).await;
    // Child 2
    increment(counter.clone(), 3).await;
}

fn main() {
    // Shared state to observe scheduler effects.
    let counter = Arc::new(Mutex::new(0u32));

    // Build executor and schedule tasks.
    let mut exec = Executor::new();
    exec.spawn(increment(counter.clone(), 5));
    exec.spawn(parent_task(counter.clone()));
    exec.spawn(async {
        // A task that does nothing but yields once.
        future::ready(()).await;
    });

    // Run the scheduler.
    exec.run();

    // Verify that all increments have been applied.
    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 10, "Counter should be 10 after all tasks");

    // Additional sanity check: executor should have no remaining tasks.
    assert!(exec.queue.is_empty(), "Scheduler queue should be empty");
    assert!(exec.tasks.iter().all(|t| t.is_none()), "All task slots should be None");
}

// === END OF FILES ===