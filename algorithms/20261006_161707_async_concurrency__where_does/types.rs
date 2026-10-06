use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
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

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}

pub struct Task {
    pub future: std::sync::Mutex<Option<BoxFuture<'static, ()>>>,

}

impl Task {
    pub fn new(fut: BoxFuture<'static, ()>) -> Self {
        Self {
            future: std::sync::Mutex::new(Some(fut)),
        }
    }
}