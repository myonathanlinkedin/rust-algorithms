use std::sync::Arc;
use std::sync::Mutex;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug, Clone, PartialEq)]
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

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

pub async fn async_task(
    id: usize,
    steps: usize,
    results: std::sync::Arc<std::sync::Mutex<Vec<usize>>>,
) {
    for _ in 0..steps {
        YieldNow::new().await;
    }
    results.lock().unwrap().push(id);
}