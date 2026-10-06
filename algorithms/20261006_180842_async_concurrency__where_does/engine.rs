use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub struct Task {
    pub id: usize,
    pub future: Pin<Box<dyn Future<Output = ()> + 'static>>,

}

pub struct Executor {
    pub queue: VecDeque<Task>,

}

impl Executor {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, future: Pin<Box<dyn Future<Output = ()> + 'static>>) {
        let id = self.queue.len();
        self.queue.push_back(Task { id, future });
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.queue.pop_front() {
            let waker = dummy_waker();
            let mut ctx = Context::from_waker(&waker);
            match task.future.as_mut().poll(&mut ctx) {
                Poll::Ready(()) => {
                    // Task completed
                }
                Poll::Pending => {
                    self.queue.push_back(task);
                }
            }
        }
    }
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

unsafe fn dummy_raw_waker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        unsafe { dummy_raw_waker() }
    }
    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    unsafe { RawWaker::new(std::ptr::null(), vtable) }
}