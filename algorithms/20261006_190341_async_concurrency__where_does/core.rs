use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct Executor {
    pub tasks: VecDeque<Pin<Box<dyn Future<Output = ()> + Send>>>,

}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: VecDeque::new(),
        }
    }

    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.push_back(Box::pin(fut));
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);
            match task.as_mut().poll(&mut cx) {
                Poll::Pending => self.tasks.push_back(task),
                Poll::Ready(()) => {}
            }
        }
    }
}

// Simple future that yields once before completing.
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

// Dummy waker required for polling futures without an async runtime.
pub fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        dummy_raw_waker()
    }
    unsafe fn no_op(_: *const ()) {}
    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(std::ptr::null(), vtable)
}

// Example async task used in tests.
pub async fn async_increment(counter: Arc<Mutex<u32>>) {
    YieldNow::new().await;
    let mut guard = counter.lock().unwrap();
    *guard += 1;
}