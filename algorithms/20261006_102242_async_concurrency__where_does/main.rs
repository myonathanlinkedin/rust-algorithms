mod types;
mod engine;

use std::sync::{Arc, Mutex};

use types::YieldNow;
use engine::Scheduler;

async fn demo_task(id: usize, log: Arc<Mutex<Vec<String>>>) {
    {
        let mut guard = log.lock().unwrap();
        guard.push(format!("task {} start", id));
    }
    YieldNow::new().await;
    {
        let mut guard = log.lock().unwrap();
        guard.push(format!("task {} resumed", id));
    }
    YieldNow::new().await;
    {
        let mut guard = log.lock().unwrap();
        guard.push(format!("task {} done", id));
    }
}

// Entry point with comprehensive assertions.
fn main() {
    let mut scheduler = Scheduler::new();
    let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    scheduler.spawn(demo_task(1, log.clone()));
    scheduler.spawn(demo_task(2, log.clone()));

    scheduler.run();

    let records = log.lock().unwrap();
    // Expected interleaving due to the explicit yields.
    let expected = vec![
        "task 1 start",
        "task 2 start",
        "task 1 resumed",
        "task 2 resumed",
        "task 1 done",
        "task 2 done",
    ];
    assert_eq!(records.len(), expected.len(), "log length mismatch");
    for (i, (actual, exp)) in records.iter().zip(expected.iter()).enumerate() {
        assert_eq!(actual, exp, "mismatch at position {}", i);
    }
}
