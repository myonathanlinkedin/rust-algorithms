use std::task::{Context, Poll};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
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

    fn poll(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.yielded {
            std::task::Poll::Ready(())
        } else {
            self.yielded = true;
            std::task::Poll::Pending
        }
    }
}

pub struct Task {
    // The future to run; wrapped in a Mutex for interior mutability.
    pub future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + 'static>>>>,

}

impl Task {
    pub fn new(fut: impl Future<Output = ()> + 'static) -> Arc<Self> {
        Arc::new(Task {
            future: Mutex::new(Some(Box::pin(fut))),
        })
    }
}
pub struct SharedResult<T> {
    pub inner: Mutex<Option<T>>,

}