use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A future that yields once before completing.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldNow {
    pub yielded: bool,

}

impl YieldNow {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

impl Default for YieldNow {
    fn default() -> Self {
        Self::new()
    }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}

/// Simple single‑threaded executor (scheduler) for `Future<Output=()>`.
pub struct Executor {
    pub tasks: VecDeque<Pin<Box<dyn Future<Output = ()> + Send>>>,

}

impl Executor {
    /// Create a new empty executor.
    pub fn new() -> Self {
        Self {
            tasks: VecDeque::new(),
        }
    }

    /// Add a future to the executor.
    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(fut));
    }

    /// Run all scheduled tasks to completion.
    pub fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);
            match task.as_mut().poll(&mut cx) {
                Poll::Ready(()) => { /* task finished */ }
                Poll::Pending => self.tasks.push_back(task),
            }
        }
    }
}

/// Create a no‑op waker suitable for polling futures that do not rely on wake‑ups.
pub fn dummy_waker() -> Waker {
    // Safety: the vtable functions do nothing and never dereference the data pointer.
    unsafe fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe fn dummy_raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    unsafe { Waker::from_raw(dummy_raw_waker()) }
}