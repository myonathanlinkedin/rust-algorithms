mod types;
mod engine;

use std::sync::{Arc, Mutex};

use engine::Executor;
use types::async_task;

fn main() {
    // Test 1: simple ordering based on steps
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut exec = Executor::new();
    exec.spawn(Box::pin(async_task(1, 2, results.clone())));
    exec.spawn(Box::pin(async_task(2, 1, results.clone())));
    exec.run();
    let res = results.lock().unwrap().clone();
    assert_eq!(res, vec![2, 1]);

    // Test 2: multiple tasks with varying steps
    let results2 = Arc::new(Mutex::new(Vec::new()));
    let mut exec2 = Executor::new();
    for i in 0..5usize {
        exec2.spawn(Box::pin(async_task(i, i % 3, results2.clone())));
    }
    exec2.run();
    let mut collected = results2.lock().unwrap().clone();
    collected.sort();
    assert_eq!(collected, (0..5).collect::<Vec<_>>());

    // Demo: show that executor runs to completion without external runtime
    let demo_results = Arc::new(Mutex::new(Vec::new()));
    let mut demo_exec = Executor::new();
    demo_exec.spawn(Box::pin(async_task(42, 5, demo_results.clone())));
    demo_exec.run();
    let demo = demo_results.lock().unwrap().clone();
    assert_eq!(demo, vec![42]);
}