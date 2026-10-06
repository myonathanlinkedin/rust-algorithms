use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A simple future that yields once before completing.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldNow(bool);

impl YieldNow {
    pub fn new() -> Self {
        YieldNow(false)
    }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.0 {
            Poll::Ready(())
        } else {
            self.0 = true;
            Poll::Pending
        }
    }
}

/// Dummy waker required for polling futures without an async runtime.
fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }

    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(std::ptr::null(), &VTABLE)
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

/// A very small executor that runs futures to completion on the current thread.
pub struct Executor {
    pub queue: Arc<Mutex<VecDeque<Pin<Box<dyn Future<Output = ()> + Send>>>>>,

}

impl Executor {
    /// Create a new executor with an empty task queue.
    pub fn new() -> Self {
        Executor {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Spawn a future onto the executor. The future must be `'static` and `Send`.
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let mut q = self.queue.lock().unwrap();
        q.push_back(Box::pin(fut));
    }

    /// Run the executor until all queued tasks have completed.
    pub fn run(&self) {
        let waker = dummy_waker();
        let mut ctx = Context::from_waker(&waker);

        loop {
            let task_opt = {
                let mut q = self.queue.lock().unwrap();
                q.pop_front()
            };

            match task_opt {
                Some(mut task) => {
                    match task.as_mut().poll(&mut ctx) {
                        Poll::Ready(()) => {
                            // Task finished; drop it.
                        }
                        Poll::Pending => {
                            // Re‑queue the task for later polling.
                            let mut q = self.queue.lock().unwrap();
                            q.push_back(task);
                        }
                    }
                }
                None => break, // No more tasks.
            }
        }
    }
}