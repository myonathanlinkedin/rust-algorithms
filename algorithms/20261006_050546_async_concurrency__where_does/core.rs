use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

pub struct Scheduler {
    pub tasks: Arc<Mutex<VecDeque<Task>>>,

}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            tasks: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Task {
            future: Box::pin(fut),
        };
        self.tasks.lock().unwrap().push_back(task);
    }

    pub fn run(&self) {
        while let Some(mut task) = self.tasks.lock().unwrap().pop_front() {
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);
            match task.future.as_mut().poll(&mut cx) {
                Poll::Pending => {
                    self.tasks.lock().unwrap().push_back(task);
                }
                Poll::Ready(_) => {}
            }
        }
    }
}

pub struct Task {
    pub future: Pin<Box<dyn Future<Output = ()> + Send + 'static>>,

}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

fn dummy_raw_waker() -> RawWaker {
    RawWaker::new(std::ptr::null(), &RAW_WAKER_VTABLE)
}

static RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
    dummy_clone,
    dummy_wake,
    dummy_wake_by_ref,
    dummy_drop,
);

unsafe fn dummy_clone(_: *const ()) -> RawWaker {
    dummy_raw_waker()
}
unsafe fn dummy_wake(_: *const ()) {}
unsafe fn dummy_wake_by_ref(_: *const ()) {}
unsafe fn dummy_drop(_: *const ()) {}
