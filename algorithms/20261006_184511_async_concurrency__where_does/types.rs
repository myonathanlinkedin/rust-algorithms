use std::collections::VecDeque;
use std::task::{Context, Poll};

use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub struct Task {
    pub id: usize,
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    pub queue: Arc<Mutex<std::collections::VecDeque<usize>>>,

}

impl Task {
    pub fn wake(&self) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(self.id);
    }
}

#[derive(Default, Debug)]
pub struct YieldNow {
    pub yielded: bool,

}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<()> {
        if self.yielded {
            std::task::Poll::Ready(())
        } else {
            self.yielded = true;
            std::task::Poll::Pending
        }
    }
}