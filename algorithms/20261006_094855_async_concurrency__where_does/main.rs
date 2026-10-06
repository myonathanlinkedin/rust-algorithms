mod types;
mod engine;

use std::future::Future;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::task::{Context, Poll};

use crate::engine::Executor;
use crate::types::Task;

/// A future that yields once before completing.
///
/// It first returns `Poll::Pending`, causing the executor to re‑schedule the task,
/// and on the second poll returns `Poll::Ready(())`.
pub struct YieldNow {
    pub yielded: bool,

}

impl YieldNow {
    pub fn new() -> Self {
        YieldNow { yielded: false }
    }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            // Wake the task so it will be polled again.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

/// Example async task that performs three steps, yielding between each step.
async fn example_task(id: usize, counter: Arc<AtomicUsize>) {
    for step in 0..3usize {
        // Simulate some work.
        println!("Task {} - step {}", id, step);
        // Yield control back to the scheduler.
        YieldNow::new().await;
    }
    // Signal completion.
    counter.fetch_add(1, Ordering::SeqCst);
}

/// Build a `Task` from an async function.
fn make_task(id: usize, counter: Arc<AtomicUsize>) -> Task {
    let fut = async move {
        example_task(id, counter).await;
    };
    Task {
        id,
        future: Box::pin(fut),
    }
}

fn main() {
    // Number of concurrent tasks to run.
    const TASK_COUNT: usize = 5;

    // Shared counter to verify that all tasks finish.
    let completed = Arc::new(AtomicUsize::new(0));

    // Create the executor.
    let mut exec = Executor::new();

    // Spawn tasks.
    for i in 0..TASK_COUNT {
        let task = make_task(i, Arc::clone(&completed));
        exec.spawn(task);
    }

    // Run the scheduler.
    exec.run();

    // All tasks should have signaled completion.
    assert_eq!(completed.load(Ordering::SeqCst), TASK_COUNT);
}
