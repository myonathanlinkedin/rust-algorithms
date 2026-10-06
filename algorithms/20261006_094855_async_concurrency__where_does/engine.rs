use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::types::Task;

/// A very small single‑threaded async executor.
///
/// It owns a queue of `Task`s and repeatedly polls them until all have completed.
pub struct Executor {
    pub queue: VecDeque<Task>,

}

impl Executor {
    /// Create a new, empty executor.
    pub fn new() -> Self {
        Executor {
            queue: VecDeque::new(),
        }
    }

    /// Add a task to the executor's queue.
    pub fn spawn(&mut self, task: Task) {
        self.queue.push_back(task);
    }

    /// Run the executor until no tasks remain.
    ///
    /// This uses a no‑op waker; tasks that return `Poll::Pending` are simply
    /// re‑queued for later polling.
    pub fn run(&mut self) {
        // A no‑op waker that does nothing when woken.
        fn noop_clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &NOOP_VTABLE)
        }
        fn noop(_: *const ()) {}
        fn noop_wake_by_ref(_: *const ()) {}
        fn noop_drop(_: *const ()) {}

        static NOOP_VTABLE: RawWakerVTable =
            RawWakerVTable::new(noop_clone, noop_wake_by_ref, noop_wake_by_ref, noop_drop);

        let raw_waker = RawWaker::new(std::ptr::null(), &NOOP_VTABLE);
        let waker = unsafe { Waker::from_raw(raw_waker) };
        let mut ctx = Context::from_waker(&waker);

        while let Some(mut task) = self.queue.pop_front() {
            match task.future.as_mut().poll(&mut ctx) {
                Poll::Ready(()) => {
                    // Task finished; drop it.
                }
                Poll::Pending => {
                    // Not ready yet; put it back for another round.
                    self.queue.push_back(task);
                }
            }
        }
    }
}
