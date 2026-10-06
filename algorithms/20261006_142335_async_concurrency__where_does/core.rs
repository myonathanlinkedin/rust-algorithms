use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

/// A future that yields once before completing.
#[derive(Debug, Clone, PartialEq)]
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

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}

/// Simple executor that runs `Future<Output = ()>` tasks to completion.
pub struct SimpleExecutor {
    pub queue: VecDeque<Pin<Box<dyn Future<Output = ()> + Send>>>,

}

impl SimpleExecutor {
    pub fn new() -> Self {
        SimpleExecutor {
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

    /// Run all queued tasks until completion.
    pub fn run(&mut self) {
        // Create a no-op waker that does nothing on wake.
        fn dummy_raw_waker() -> RawWaker {
            fn no_op(_: *const ()) {}
            fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
            const VTABLE: RawWakerVTable =
                RawWakerVTable::new(clone, no_op, no_op, no_op);
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        let waker = unsafe { Waker::from_raw(dummy_raw_waker()) };
        let mut ctx = Context::from_waker(&waker);

        while let Some(mut task) = self.queue.pop_front() {
            match task.as_mut().poll(&mut ctx) {
                Poll::Ready(()) => { /* task finished */ }
                Poll::Pending => {
                    // Re-queue the task for later polling.
                    self.queue.push_back(task);
                }
            }
        }
    }
}