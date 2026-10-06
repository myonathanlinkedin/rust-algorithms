use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A simple future that yields once before completing.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldNow {
    pub yielded: bool,

}

impl YieldNow {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}

/// Creates a dummy waker that does nothing. Required for polling futures.
fn dummy_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        raw_waker()
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    fn raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(raw_waker()) }
}

/// A very small single‑threaded executor that runs `Future<Output = ()>` tasks.
pub struct Executor {
    pub queue: VecDeque<Pin<Box<dyn Future<Output = ()> + Send>>>,

}

impl Executor {
    /// Construct a new empty executor.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Add a future to the executor.
    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.queue.push_back(Box::pin(fut));
    }

    /// Run until all queued tasks have completed.
    pub fn run(&mut self) {
        while let Some(mut task) = self.queue.pop_front() {
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);
            match task.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {} // task finished
                Poll::Pending => self.queue.push_back(task), // re‑queue for later polling
            }
        }
    }
}

/// An example async workload that increments a shared counter `times` times,
/// yielding after each increment.
pub async fn async_increment(counter: std::sync::Arc<std::sync::Mutex<usize>>, times: usize) {
    for _ in 0..times {
        {
            let mut guard = counter.lock().unwrap();
            *guard += 1;
        }
        // Yield control back to the scheduler.
        YieldNow::new().await;
    }
}