mod types;
mod engine;

use std::sync::Arc;

use crate::engine::Executor;
use crate::types::{YieldNow, SharedResult};

async fn async_compute() -> i32 {
    YieldNow::new().await;
    2 + 3
}

fn main() {
    // Create the executor.
    let executor = Executor::new();

    // Shared result holder for the async task.
    let result = Arc::new(SharedResult { inner: std::sync::Mutex::new(None) });

    // Spawn a task that computes a value.
    {
        let result_clone = result.clone();
        executor.spawn(async move {
            let val = async_compute().await;
            *result_clone.inner.lock().unwrap() = Some(val);
        });
    }

    // Spawn a second task that yields twice before completing.
    {
        let result_clone = result.clone();
        executor.spawn(async move {
            YieldNow::new().await;
            YieldNow::new().await;
            *result_clone.inner.lock().unwrap() = Some(42);
        });
    }

    // Run the executor until all tasks are complete.
    executor.run();

    // Verify that the first task produced the expected result.
    let val = *result.inner.lock().unwrap();
    assert_eq!(val, Some(42), "The last completed task should set the result to 42");

    // Additional sanity check: ensure the executor's queue is empty after run.
    // (We cannot directly access the queue, but if run returned, it means empty.)
    println!("All async tasks completed successfully.");
}