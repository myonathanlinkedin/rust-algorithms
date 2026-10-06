mod types;
mod engine;

use std::sync::{Arc, Mutex};

use engine::Executor;
use types::YieldOnce;

async fn compute(a: i32, b: i32, out: Arc<Mutex<Option<i32>>>) {
    // Force a yield to test the scheduler.
    YieldOnce::new().await;
    let sum = a + b;
    *out.lock().unwrap() = Some(sum);
}

fn main() {
    let exec = Executor::new();

    let result1 = Arc::new(Mutex::new(None));
    let result2 = Arc::new(Mutex::new(None));

    exec.spawn(compute(2, 3, result1.clone()));
    exec.spawn(compute(10, 5, result2.clone()));

    exec.run();

    assert_eq!(*result1.lock().unwrap(), Some(5));
    assert_eq!(*result2.lock().unwrap(), Some(15));

    // Additional sanity check: ensure that a task that yields twice works.
    let counter = Arc::new(Mutex::new(0));
    async fn double_yield(counter: Arc<Mutex<i32>>) {
        // First yield
        YieldOnce::new().await;
        // Second yield
        YieldOnce::new().await;
        *counter.lock().unwrap() += 1;
    }

    exec.spawn(double_yield(counter.clone()));
    exec.run();

    assert_eq!(*counter.lock().unwrap(), 1);
}