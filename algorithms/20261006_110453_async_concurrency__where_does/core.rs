use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

#[derive(Debug, Clone, PartialEq)]
pub struct Counter {
    pub current: usize,
    pub max: usize,

}

impl Counter {
    pub fn new(max: usize) -> Self {
        Counter { current: 0, max }
    }
}

impl Future for Counter {
    type Output = usize;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.current >= self.max {
            Poll::Ready(self.current)
        } else {
            self.current += 1;
            // Wake the executor to poll again.
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub struct SimpleExecutor {
    pub tasks: VecDeque<Pin<Box<dyn Future<Output = ()> + 'static>>>,

}

impl SimpleExecutor {
    pub fn new() -> Self {
        SimpleExecutor {
            tasks: VecDeque::new(),
        }
    }

    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.push_back(Box::pin(fut));
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);
            match task.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {} // Task completed.
                Poll::Pending => self.tasks.push_back(task), // Re-queue.
            }
        }
    }
}

// A no-op waker used by the executor.
fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

unsafe fn dummy_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &DUMMY_VTABLE)
}

unsafe fn clone(_: *const ()) -> RawWaker {
    dummy_raw_waker()
}
unsafe fn wake(_: *const ()) {}
unsafe fn wake_by_ref(_: *const ()) {}
unsafe fn drop(_: *const ()) {}

static DUMMY_VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);