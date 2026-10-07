use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Simple task wrapper holding a boxed future.
pub struct Task {
    pub future: Arc<Mutex<Option<Pin<Box<dyn Future<Output = ()> + 'static>>>>>,

}

impl Task {
    fn new<F>(future: F) -> Self
    where
        F: Future<Output = ()> + 'static,
    {
        Task {
            future: Arc::new(Mutex::new(Some(Box::pin(future)))),
        }
    }

    fn poll(&self, cx: &mut Context<'_>) -> Poll<()> {
        let mut guard = self.future.lock().unwrap();
        if let Some(mut fut) = guard.take() {
            match fut.as_mut().poll(cx) {
                Poll::Ready(()) => Poll::Ready(()),
                Poll::Pending => {
                    *guard = Some(fut);
                    Poll::Pending
                }
            }
        } else {
            Poll::Ready(())
        }
    }

    fn waker(&self) -> Waker {
        // Dummy waker; executor drives progress manually.
        unsafe { Waker::from_raw(dummy_raw_waker()) }
    }
}

/// Dummy RawWaker implementation – does nothing.
unsafe fn dummy_waker_clone(data: *const ()) -> RawWaker {
    dummy_raw_waker()
}
unsafe fn dummy_waker_wake(_data: *const ()) {}
unsafe fn dummy_waker_wake_by_ref(_data: *const ()) {}
unsafe fn dummy_waker_drop(_data: *const ()) {}

fn dummy_raw_waker() -> RawWaker {
    RawWaker::new(
        std::ptr::null(),
        &RawWakerVTable::new(
            dummy_waker_clone,
            dummy_waker_wake,
            dummy_waker_wake_by_ref,
            dummy_waker_drop,
        ),
    )
}

/// Single‑threaded executor (the scheduler).
pub struct Executor {
    pub queue: VecDeque<Task>,

}

impl Executor {
    /// Create a new empty executor.
    pub fn new() -> Self {
        Executor {
            queue: VecDeque::new(),
        }
    }

    /// Spawn a future onto the executor.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.queue.push_back(Task::new(future));
    }

    /// Run until all tasks have completed.
    pub fn run(&mut self) {
        while let Some(task) = self.queue.pop_front() {
            let waker = task.waker();
            let mut ctx = Context::from_waker(&waker);
            match task.poll(&mut ctx) {
                Poll::Ready(()) => {} // task finished
                Poll::Pending => {
                    self.queue.push_back(task);
                }
            }
        }
    }
}