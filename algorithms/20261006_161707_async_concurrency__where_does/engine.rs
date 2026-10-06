use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::types::{BoxFuture, Task};

struct TaskWaker {
    task: Arc<Task>,
    queue: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

fn raw_waker(data: *const ()) -> RawWaker {
    RawWaker::new(data, &VTABLE)
}

unsafe fn waker_clone(data: *const ()) -> RawWaker {
    // Increase the Arc count
    let arc = Arc::<TaskWaker>::from_raw(data as *const TaskWaker);
    let _ = arc.clone();
    // Forget the original Arc to keep the count unchanged
    std::mem::forget(arc);
    raw_waker(data)
}

unsafe fn waker_wake(data: *const ()) {
    // Take ownership and push the task back onto the queue
    let waker = Arc::<TaskWaker>::from_raw(data as *const TaskWaker);
    waker
        .queue
        .lock()
        .unwrap()
        .push_back(waker.task.clone());
    // Dropping `waker` decreases the ref count
}

unsafe fn waker_wake_by_ref(data: *const ()) {
    // Clone the Arc without taking ownership
    let waker = Arc::<TaskWaker>::from_raw(data as *const TaskWaker);
    waker
        .queue
        .lock()
        .unwrap()
        .push_back(waker.task.clone());
    // Forget the original to keep the count unchanged
    std::mem::forget(waker);
}

unsafe fn waker_drop(data: *const ()) {
    // Decrease the Arc count
    let _ = Arc::<TaskWaker>::from_raw(data as *const TaskWaker);
}

static VTABLE: RawWakerVTable =
    RawWakerVTable::new(waker_clone, waker_wake, waker_wake_by_ref, waker_drop);

fn make_waker(task: Arc<Task>, queue: Arc<Mutex<VecDeque<Arc<Task>>>>) -> Waker {
    let waker_data = Arc::new(TaskWaker { task, queue });
    unsafe { Waker::from_raw(raw_waker(Arc::into_raw(waker_data) as *const ())) }
}
pub struct Executor {
    pub queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl Executor {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task::new(Box::pin(fut)));
        self.queue.lock().unwrap().push_back(task);
    }

    pub fn run(&self) {
        while let Some(task) = self.queue.lock().unwrap().pop_front() {
            let waker = make_waker(task.clone(), self.queue.clone());
            let mut ctx = Context::from_waker(&waker);
            let mut future_opt = task.future.lock().unwrap();
            if let Some(mut fut) = future_opt.take() {
                match fut.as_mut().poll(&mut ctx) {
                    Poll::Pending => {
                        *future_opt = Some(fut);
                    }
                    Poll::Ready(()) => {
                        // Future completed; drop it.
                    }
                }
            }
        }
    }
}