use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::types::{Task, YieldNow};

pub struct Executor {
    pub tasks: Mutex<Vec<Option<Arc<Task>>>>,
    pub queue: Arc<Mutex<VecDeque<usize>>>,

}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: Mutex::new(Vec::new()),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn spawn(&self, fut: impl Future<Output = ()> + Send + 'static) {
        let mut tasks = self.tasks.lock().unwrap();
        let id = tasks.len();
        let task = Arc::new(Task {
            id,
            future: Mutex::new(Box::pin(fut)),
            queue: self.queue.clone(),
        });
        tasks.push(Some(task.clone()));
        let mut q = self.queue.lock().unwrap();
        q.push_back(id);
    }

    pub fn run(&self) {
        loop {
            let next = {
                let mut q = self.queue.lock().unwrap();
                q.pop_front()
            };
            match next {
                Some(id) => {
                    let task_opt = {
                        let tasks = self.tasks.lock().unwrap();
                        tasks.get(id).and_then(|opt| opt.clone())
                    };
                    if let Some(task) = task_opt {
                        let waker = create_waker(task.clone());
                        let mut ctx = Context::from_waker(&waker);
                        let mut future_guard = task.future.lock().unwrap();
                        match future_guard.as_mut().poll(&mut ctx) {
                            Poll::Ready(()) => {
                                let mut tasks = self.tasks.lock().unwrap();
                                tasks[id] = None;
                            }
                            Poll::Pending => {
                                // task will be rescheduled by its waker
                            }
                        }
                    }
                }
                None => {
                    let tasks = self.tasks.lock().unwrap();
                    if tasks.iter().all(|opt| opt.is_none()) {
                        break;
                    } else {
                        // No ready tasks but some are pending; break to avoid deadlock in this simple executor
                        break;
                    }
                }
            }
        }
    }
}

fn raw_waker_clone(data: *const ()) -> RawWaker {
    unsafe { RawWaker::new(data, &VTABLE) }
}

fn raw_waker_wake(data: *const ()) {
    unsafe {
        // Take ownership of the Arc and drop after waking
        let task = Arc::from_raw(data as *const Task);
        task.wake();
        // task dropped here
    }
}

fn raw_waker_wake_by_ref(data: *const ()) {
    unsafe {
        // Recreate Arc, clone for wake, then forget original to keep it alive
        let task = Arc::from_raw(data as *const Task);
        let task_clone = task.clone();
        task.wake();
        std::mem::forget(task);
        // task_clone dropped after wake
    }
}

fn raw_waker_drop(data: *const ()) {
    unsafe {
        // Decrease the Arc count
        let _ = Arc::from_raw(data as *const Task);
    }
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    raw_waker_clone,
    raw_waker_wake,
    raw_waker_wake_by_ref,
    raw_waker_drop,
);

pub fn create_waker(task: Arc<Task>) -> Waker {
    unsafe {
        let raw = RawWaker::new(Arc::into_raw(task) as *const (), &VTABLE);
        Waker::from_raw(raw)
    }
}