use std::collections::VecDeque;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    pub queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl Task {
    fn poll(self: Arc<Self>) {
        let waker = task_waker(self.clone());
        let mut cx = Context::from_waker(&waker);
        let mut fut = self.future.lock().unwrap();
        let _ = fut.as_mut().poll(&mut cx);
        // If pending, the waker will re‑queue the task.
    }
}

pub struct Executor {
    pub queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl Executor {
    pub fn new() -> Self {
        Executor {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
            queue: self.queue.clone(),
        });
        self.queue.lock().unwrap().push_back(task);
    }

    pub fn run(&self) {
        while let Some(task) = self.queue.lock().unwrap().pop_front() {
            task.poll();
        }
    }
}

// ----- Waker implementation -----
fn task_waker(task: Arc<Task>) -> Waker {
    unsafe { Waker::from_raw(raw_waker(task)) }
}

unsafe fn raw_waker(task: Arc<Task>) -> RawWaker {
    let data = Arc::into_raw(task) as *const ();
    RawWaker::new(data, &VTABLE)
}

unsafe fn raw_waker_clone(data: *const ()) -> RawWaker {
    let arc = Arc::from_raw(data as *const Task);
    let _clone = arc.clone(); // increments ref count
    mem::forget(arc);
    mem::forget(_clone);
    RawWaker::new(data, &VTABLE)
}

unsafe fn raw_waker_wake(data: *const ()) {
    let task = Arc::from_raw(data as *const Task);
    task.queue.lock().unwrap().push_back(task.clone());
    // original `task` is dropped here, leaving the clone in the queue
}

unsafe fn raw_waker_wake_by_ref(data: *const ()) {
    let arc = Arc::from_raw(data as *const Task);
    let task_clone = arc.clone();
    arc.queue.lock().unwrap().push_back(task_clone);
    mem::forget(arc); // keep original alive
}

unsafe fn raw_waker_drop(data: *const ()) {
    // Decrease the strong count
    let _ = Arc::from_raw(data as *const Task);
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    raw_waker_clone,
    raw_waker_wake,
    raw_waker_wake_by_ref,
    raw_waker_drop,
);