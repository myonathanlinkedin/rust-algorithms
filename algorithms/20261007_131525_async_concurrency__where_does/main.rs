mod core;

use core::Executor;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            Poll::Pending
        }
    }
}

// Async task that increments a shared counter after yielding once.
async fn async_increment(counter: Arc<Mutex<i32>>) {
    YieldNow(false).await;
    let mut lock = counter.lock().unwrap();
    *lock += 1;
}

// Async task that records its identifier after two yields.
async fn record_id(id: i32, out: Arc<Mutex<Vec<i32>>>) {
    YieldNow(false).await;
    YieldNow(false).await;
    out.lock().unwrap().push(id);
}

// ----- Tests -----
fn test_executor_basic() {
    let exec = Executor::new();
    let counter = Arc::new(Mutex::new(0));
    for _ in 0..5 {
        exec.spawn(async_increment(counter.clone()));
    }
    exec.run();
    assert_eq!(*counter.lock().unwrap(), 5);
}

fn test_executor_multiple_tasks() {
    let exec = Executor::new();
    let results = Arc::new(Mutex::new(Vec::new()));
    for i in 0..3 {
        exec.spawn(record_id(i, results.clone()));
    }
    exec.run();
    let mut collected = results.lock().unwrap().clone();
    collected.sort(); // order is nondeterministic
    assert_eq!(collected, vec![0, 1, 2]);
}

fn main() {
    test_executor_basic();
    test_executor_multiple_tasks();
}