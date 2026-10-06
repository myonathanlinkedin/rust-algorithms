use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::types::Task;

/// Internal data stored inside a waker. Holds a reference to the task to wake and the executor's queue.
struct WakeData {
    task: Arc<Task>,
    executor: Arc<ExecutorInner>,
}

/// Core executor data shared between wakers and the executor itself.
struct ExecutorInner {
    queue: Mutex<VecDeque<Arc<Task>>>,
}

/// Public executor handle.
pub struct Executor {
    pub inner: Arc<ExecutorInner>,

}

impl Executor {
    /// Create a new executor.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ExecutorInner {
                queue: Mutex::new(VecDeque::new()),
            }),
        }
    }

    /// Spawn a future onto the executor.
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
        });
        self.inner.queue.lock().unwrap().push_back(task);
    }

    /// Run the executor until no tasks remain.
    pub fn run(&self) {
        while let Some(task) = self.inner.queue.lock().unwrap().pop_front() {
            // Build a waker for this task.
            let waker = waker_from_task(task.clone(), self.inner.clone());
            let mut ctx = Context::from_waker(&waker);

            // Poll the future.
            let mut future_guard = task.future.lock().unwrap();
            match future_guard.as_mut().poll(&mut ctx) {
                Poll::Ready(()) => {
                    // Task completed; drop it.
                }
                Poll::Pending => {
                    // The task will be re-queued by its waker.
                }
            }
        }
    }
}

/// Construct a `Waker` from a task and executor reference.
fn waker_from_task(task: Arc<Task>, exec: Arc<ExecutorInner>) -> Waker {
    // Allocate WakeData on the heap and turn it into a raw pointer.
    let data = Box::into_raw(Box::new(WakeData { task, executor: exec }));
    unsafe { Waker::from_raw(RawWaker::new(data as *const (), &VTABLE)) }
}

// Raw waker vtable implementation.
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    raw_waker_clone,
    raw_waker_wake,
    raw_waker_wake_by_ref,
    raw_waker_drop,
);

unsafe fn raw_waker_clone(data: *const ()) -> RawWaker {
    // Recreate the WakeData reference to clone its Arc fields.
    let wd = &*(data as *const WakeData);
    let new = Box::new(WakeData {
        task: wd.task.clone(),
        executor: wd.executor.clone(),
    });
    RawWaker::new(Box::into_raw(new) as *const (), &VTABLE)
}

unsafe fn raw_waker_wake(data: *const ()) {
    // Take ownership of the WakeData, then re-queue the task.
    let wd = Box::from_raw(data as *mut WakeData);
    wd.executor
        .queue
        .lock()
        .unwrap()
        .push_back(wd.task.clone());
    // `wd` is dropped here, decreasing the Arc counts.
}

unsafe fn raw_waker_wake_by_ref(data: *const ()) {
    // Borrow the WakeData and re-queue the task without taking ownership.
    let wd = &*(data as *const WakeData);
    wd.executor
        .queue
        .lock()
        .unwrap()
        .push_back(wd.task.clone());
}

unsafe fn raw_waker_drop(data: *const ()) {
    // Reclaim the boxed WakeData.
    let _ = Box::from_raw(data as *mut WakeData);
}