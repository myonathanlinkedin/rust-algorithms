use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

pub struct Scheduler {
    pub queue: VecDeque<Pin<Box<dyn Future<Output = ()> + 'static>>>,

}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            queue: VecDeque::new(),
        }
    }

    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.queue.push_back(Box::pin(fut));
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.queue.pop_front() {
            // Create a dummy waker that does nothing.
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);
            match task.as_mut().poll(&mut cx) {
                Poll::Ready(()) => {
                    // Task completed; drop it.
                }
                Poll::Pending => {
                    // Re‑queue for later polling.
                    self.queue.push_back(task);
                }
            }
        }
    }
}

// Dummy waker implementation.
fn dummy_waker() -> Waker {
    struct DummyWaker;
    impl Wake for DummyWaker {
        fn wake(self: Arc<Self>) {
            // No-op: the scheduler manually re‑polls tasks.
        }
    }
    let arc = Arc::new(DummyWaker);
    Waker::from(arc)
}

// Placeholder main to satisfy file‑level requirement.
fn main() {}
