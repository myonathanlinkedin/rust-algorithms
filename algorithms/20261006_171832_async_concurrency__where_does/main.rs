mod types;
mod engine;

use std::sync::{Arc, Mutex};

use engine::Executor;
use types::{YieldNow, Task};

async fn async_task(name: &'static str, log: Arc<Mutex<Vec<&'static str>>>) {
    // Record start.
    log.lock().unwrap().push(name);
    // Yield once to allow interleaving.
    YieldNow::new().await;
    // Record completion.
    log.lock().unwrap().push(name);
}

fn main() {
    // Shared log to capture execution order.
    let log: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

    let mut exec = Executor::new();

    // Spawn two tasks that will interleave.
    exec.spawn(async_task("task1", log.clone()));
    exec.spawn(async_task("task2", log.clone()));

    // Run the executor to completion.
    exec.run();

    // Expected order: start of task1, start of task2, end of task1, end of task2
    // because each task yields once.
    let result = log.lock().unwrap().clone();
    let expected = vec!["task1", "task2", "task1", "task2"];
    assert_eq!(result, expected, "Scheduler interleaving did not match expectation");

    // Additional sanity check: ensure no tasks remain.
    assert!(exec.tasks.iter().all(|slot| slot.is_none()), "All tasks should be completed");
}