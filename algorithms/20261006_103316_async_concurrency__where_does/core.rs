use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct SimpleExecutor {
    pub tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,

}

impl SimpleExecutor {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.push(Box::pin(fut));
    }

    pub fn run(&mut self) {
        while !self.tasks.is_empty() {
            let mut i = 0usize;
            while i < self.tasks.len() {
                let waker = dummy_waker();
                let mut cx = Context::from_waker(&waker);
                match self.tasks[i].as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        self.tasks.swap_remove(i);
                    }
                    Poll::Pending => {
                        i += 1;
                    }
                }
            }
        }
    }
}

// Dummy waker that does nothing.
fn dummy_waker() -> std::task::Waker {
    use std::task::{RawWaker, RawWakerVTable};

    unsafe fn clone(_: *const ()) -> RawWaker {
        raw_waker()
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    fn raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }

    static VTABLE: RawWakerVTable =
        RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe { std::task::Waker::from_raw(raw_waker()) }
}

// Future that yields once.
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

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            Poll::Pending
        }
    }
}

// Example async computation.
pub async fn async_add(a: i32, b: i32) -> i32 {
    YieldNow::new().await;
    a + b
}