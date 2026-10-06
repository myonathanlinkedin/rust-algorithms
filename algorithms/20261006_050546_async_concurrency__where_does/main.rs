mod core;

use core::Scheduler;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    // Simple test: spawn two async tasks that increment a counter
    let counter = Arc::new(Mutex::new(0));
    let counter1 = counter.clone();
    let counter2 = counter.clone();

    let scheduler = Scheduler::new();

    scheduler.spawn(async move {
        for _ in 0..5 {
            async_yield().await;
            let mut val = counter1.lock().unwrap();
            *val += 1;
        }
    });

    scheduler.spawn(async move {
        for _ in 0..5 {
            async_yield().await;
            let mut val = counter2.lock().unwrap();
            *val += 1;
        }
    });

    let start = Instant::now();
    scheduler.run();
    let duration = start.elapsed();

    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 10, "Counter should be 10 after tasks complete");

    // Benchmark: measure time to run 1000 trivial tasks
    let scheduler = Scheduler::new();
    for _ in 0..1000 {
        scheduler.spawn(async { /* trivial */ });
    }
    let start = Instant::now();
    scheduler.run();
    let bench_duration = start.elapsed();
    println!("Ran 1000 trivial tasks in {:?}", bench_duration);
}

// Simple async yield future
struct Yield;

impl std::future::Future for Yield {
    type Output = ();
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        cx.waker().wake_by_ref();
        std::task::Poll::Ready(())
    }
}

fn async_yield() -> Yield {
    Yield
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_scheduler_basic() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();
        let scheduler = Scheduler::new();
        scheduler.spawn(async move {
            for _ in 0..3 {
                async_yield().await;
                let mut val = counter_clone.lock().unwrap();
                *val += 1;
            }
        });
        scheduler.run();
        assert_eq!(*counter.lock().unwrap(), 3);
    }

    #[test]
    fn test_scheduler_multiple_tasks() {
        let counter = Arc::new(Mutex::new(0
));

}
}
