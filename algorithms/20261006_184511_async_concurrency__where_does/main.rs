use std::future::Future;

mod types;
mod engine;

use std::sync::{Arc, Mutex};

use crate::engine::Executor;
use crate::types::{YieldNow};

fn async_increment(counter: Arc<Mutex<i32>>, times: i32) -> impl std::future::Future<Output = ()> + Send {
    async move {
        for _ in 0..times {
            {
                let mut lock = counter.lock().unwrap();
                *lock += 1;
            }
            YieldNow::default().await;
        }
    }
}

fn async_logger(log: Arc<Mutex<Vec<&'static str>>>, id: &'static str) -> impl std::future::Future<Output = ()> + Send {
    async move {
        log.lock().unwrap().push(id);
        YieldNow::default().await;
        log.lock().unwrap().push(id);
    }
}

fn main() {
    let exec = Executor::new();

    // Test simple counter increment with yielding
    let counter = Arc::new(Mutex::new(0));
    exec.spawn(async_increment(counter.clone(), 5));
    exec.spawn(async_increment(counter.clone(), 5));
    exec.run();
    assert_eq!(*counter.lock().unwrap(), 10);

    // Test interleaved logging to demonstrate scheduler interleaving
    let log = Arc::new(Mutex::new(Vec::new()));
    exec.spawn(async_logger(log.clone(), "A"));
    exec.spawn(async_logger(log.clone(), "B"));
    exec.run();

    let recorded = log.lock().unwrap().clone();
    // Expected pattern: A, B, A, B (or B, A, B, A) depending on scheduling order
    // Verify that each identifier appears exactly twice and order is interleaved
    assert_eq!(recorded.len(), 4);
    assert_eq!(recorded.iter().filter(|&&s| s == "A").count(), 2);
    assert_eq!(recorded.iter().filter(|&&s| s == "B").count(), 2);
    // Ensure not all As then all Bs
    let first_two = &recorded[0..2];
    assert!(!(first_two[0] == "A" && first_two[1] == "A"));
    assert!(!(first_two[0] == "B" && first_two[1] == "B"));
}