use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// A simple future that yields once before completing.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldOnce {
    pub yielded: bool,

}

impl YieldOnce {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

impl Future for YieldOnce {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if !self.yielded {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

/// Returns a no‑op waker suitable for polling futures in a single‑threaded executor.
fn dummy_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    unsafe fn dummy_raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

/// A task that owns a boxed future.
pub struct Task {
    pub future: Pin<Box<dyn Future<Output = ()> + Send>>,

}

impl Task {
    fn new(fut: impl Future<Output = ()> + Send + 'static) -> Self {
        Self {
            future: Box::pin(fut),
        }
    }
}

/// A minimal single‑threaded async executor.
pub struct Executor {
    pub queue: VecDeque<Task>,

}

impl Executor {
    /// Creates a new empty executor.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    /// Spawns a future onto the executor.
    pub fn spawn(&mut self, fut: impl Future<Output = ()> + Send + 'static) {
        self.queue.push_back(Task::new(fut));
    }

    /// Runs tasks until the queue is empty.
    pub fn run(&mut self) {
        let waker = dummy_waker();
        let mut ctx = Context::from_waker(&waker);
        while let Some(mut task) = self.queue.pop_front() {
            match task.future.as_mut().poll(&mut ctx) {
                Poll::Ready(()) => {} // task finished
                Poll::Pending => {
                    self.queue.push_back(task);
                }
            }
        }
    }
}

/// Example async computation used in tests.
pub async fn compute_sum() -> i32 {
    let mut sum = 0;
    for i in 0..3 {
        YieldOnce::new().await;
        sum += i;
    }
    sum
}