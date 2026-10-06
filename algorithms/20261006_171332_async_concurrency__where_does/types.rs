use std::task::{Context, Poll};
pub use std::future::Future;
pub use std::pin::Pin;
pub use std::sync::Arc;
pub use std::sync::atomic::{AtomicUsize, Ordering};

pub struct SimpleTask {
    pub future: Pin<Box<dyn Future<Output = ()> + 'static>>,

}

impl SimpleTask {
    pub fn new<F>(fut: F) -> Self
    where
        F: Future<Output = ()> + 'static,
    {
        SimpleTask {
            future: Box::pin(fut),
        }
    }
}

/// A future that becomes ready after being polled `target` times.
/// When it completes it increments the provided `counter`.
pub struct CounterFuture {
    pub target: usize,
    pub polls: usize,
    pub counter: Arc<AtomicUsize>,

}

impl CounterFuture {
    pub fn new(target: usize, counter: Arc<AtomicUsize>) -> Self {
        CounterFuture {
            target,
            polls: 0,
            counter,
        }
    }
}

impl Future for CounterFuture {
    type Output = ();

    fn poll(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        self.polls += 1;
        if self.polls >= self.target {
            self.counter.fetch_add(1, Ordering::SeqCst);
            std::task::Poll::Ready(())
        } else {
            std::task::Poll::Pending
        }
    }
}