use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// Simple single‑threaded async executor. The scheduler lives inside this struct.
pub struct Executor {
    // Each slot holds an optional future; `None` means the task has completed.
    pub tasks: Vec<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    // Queue of task indices ready to be polled.
    pub queue: VecDeque<usize>,

}

impl Executor {
    /// Create a new, empty executor.
    pub fn new() -> Self {
        Executor {
            tasks: Vec::new(),
            queue: VecDeque::new(),
        }
    }

    /// Spawn a new future onto the executor.
    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let idx = self.tasks.len();
        self.tasks.push(Some(Box::pin(fut)));
        self.queue.push_back(idx);
    }

    /// Run the scheduler until no tasks remain.
    pub fn run(&mut self) {
        while let Some(idx) = self.queue.pop_front() {
            // Take the future out of the slot; if it's already None we skip.
            if let Some(mut fut) = self.tasks[idx].take() {
                // Create a no‑op waker for polling.
                let waker = dummy_waker();
                let mut cx = Context::from_waker(&waker);
                match fut.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        // Task finished; drop it.
                    }
                    Poll::Pending => {
                        // Not ready yet; put it back and re‑queue.
                        self.tasks[idx] = Some(fut);
                        self.queue.push_back(idx);
                    }
                }
            }
        }
    }
}

// Helper to build a no‑op waker.
fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(std::ptr::null(), vtable)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}