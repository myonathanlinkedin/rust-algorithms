mod types;
mod engine;

use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

use types::{CounterFuture, SimpleTask};
use engine::Executor;

fn main() {
    // Shared counters to verify task completions.
    let counter_a = Arc::new(AtomicUsize::new(0));
    let counter_b = Arc::new(AtomicUsize::new(0));

    // Create two futures that complete after different numbers of polls.
    let fut_a = CounterFuture::new(3, Arc::clone(&counter_a));
    let fut_b = CounterFuture::new(5, Arc::clone(&counter_b));

    // Wrap them into SimpleTask objects.
    let task_a = SimpleTask::new(fut_a);
    let task_b = SimpleTask::new(fut_b);

    // Initialize executor and spawn tasks.
    let mut exec = Executor::new();
    exec.spawn(task_a);
    exec.spawn(task_b);

    // Run the scheduler (executor).
    exec.run();

    // Assertions: both counters should have been incremented exactly once.
    assert_eq!(counter_a.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert_eq!(counter_b.load(std::sync::atomic::Ordering::SeqCst), 1);
}

// Additional unit tests.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_executor_completes_all_tasks() {
        let counter1 = Arc::new(AtomicUsize::new(0));
        let counter2 = Arc::new(AtomicUsize::new(0));
        let counter3 = Arc::new(AtomicUsize::new(0));

        let task1 = SimpleTask::new(CounterFuture::new(1, Arc::clone(&counter1)));
        let task2 = SimpleTask::new(CounterFuture::new(2, Arc::clone(&counter2)));
        let task3 = SimpleTask::new(CounterFuture::new(4, Arc::clone(&counter3)));

        let mut exec = Executor::new();
        exec.spawn(task1);
        exec.spawn(task2);
        exec.spawn(task3);
        exec.run();

        assert_eq!(counter1.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(counter2.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert_eq!(counter3.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}