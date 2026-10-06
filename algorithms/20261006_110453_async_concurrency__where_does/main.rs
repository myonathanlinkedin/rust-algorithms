mod core;
use core::{Counter, SimpleExecutor};
use std::sync::{Arc, Mutex};

fn test_scheduler() {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut exec = SimpleExecutor::new();

    // Task 1: counts to 5.
    {
        let results = Arc::clone(&results);
        exec.spawn(async move {
            let val = Counter::new(5).await;
            results.lock().unwrap().push(val);
        });
    }

    // Task 2: counts to 3.
    {
        let results = Arc::clone(&results);
        exec.spawn(async move {
            let val = Counter::new(3).await;
            results.lock().unwrap().push(val);
        });
    }

    exec.run();

    let mut collected = results.lock().unwrap().clone();
    collected.sort_unstable();
    assert_eq!(collected, vec![3, 5]);
}

fn main() {
    test_scheduler();
    // Additional sanity check.
    let mut exec = SimpleExecutor::new();
    let flag = Arc::new(Mutex::new(false));
    {
        let flag = Arc::clone(&flag);
        exec.spawn(async move {
            // Simple future that immediately resolves.
            *flag.lock().unwrap() = true;
        });
    }
    exec.run();
    assert!(*flag.lock().unwrap());
}