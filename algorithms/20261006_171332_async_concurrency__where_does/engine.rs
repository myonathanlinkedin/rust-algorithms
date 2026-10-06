use std::collections::VecDeque;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::types::SimpleTask;

pub struct Executor {
    pub queue: VecDeque<SimpleTask>,

}

impl Executor {
    pub fn new() -> Self {
        Executor {
            queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: SimpleTask) {
        self.queue.push_back(task);
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.queue.pop_front() {
            // Create a no-op waker for polling.
            let waker = noop_waker();
            let mut cx = Context::from_waker(&waker);
            match task.future.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    // Task completed; drop it.
                }
                Poll::Pending => {
                    // Re-queue the task for later polling.
                    self.queue.push_back(task);
                }
            }
        }
    }
}

// A no-op waker that does nothing on wake.
fn noop_raw_waker() -> RawWaker {
    fn clone(_: *const ()) -> RawWaker {
        noop_raw_waker()
    }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}

    let vtable = &RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    RawWaker::new(std::ptr::null(), vtable)
}

fn noop_waker() -> Waker {
    unsafe { Waker::from_raw(noop_raw_waker()) }
}