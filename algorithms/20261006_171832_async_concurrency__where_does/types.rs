use std::task::{Context, Poll};

use std::future::Future;
use std::pin::Pin;

pub struct Task {
    pub future: Pin<Box<dyn Future<Output = ()> + 'static>>,

}

// Simple future that yields once before completing.
#[derive(Debug, Clone, PartialEq, Default)]
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

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if !self.yielded {
            self.yielded = true;
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        } else {
            std::task::Poll::Ready(())
        }
    }
}