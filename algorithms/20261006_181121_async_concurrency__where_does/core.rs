use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A task that holds a boxed future and a reference to the executor's queue.
pub struct Task {
    /// The future to be polled. Wrapped in a Mutex because the executor may
    /// need mutable access while the task is shared.
    pub future: Mutex<Option<Box<dyn Future<Output = ()> + Send>>>,
    /// Queue where the task should be re‑enqueued when woken.
    pub queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl Task {
    fn new<F>(future: F, queue: Arc<Mutex<VecDeque<Arc<Task>>>>) -> Arc<Self>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Arc::new(Task {
            future: Mutex::new(Some(Box::new(future))),
            queue,
        })
    }
}

/// Simple single‑threaded executor that owns a task queue.
pub struct Executor {
    /// Shared queue of pending tasks.
    pub queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl Executor {
    /// Create a new empty executor.
    pub fn new() -> Self {
        Executor {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Spawn a future onto the executor.
    pub fn spawn<F>(&self, future: F) -> Arc<Task>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Task::new(future, self.queue.clone());
        self.queue.lock().unwrap().push_back(task.clone());
        task
    }

    /// Run the executor until the task queue is empty.
    pub fn run(&self) {
        while let Some(task) = self.queue.lock().unwrap().pop_front() {
            // Take the future out for polling.
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut fut) = future_slot.take() {
                // Build a waker that knows how to re‑queue this task.
                let waker = task_waker(task.clone());
                let mut cx = Context::from_waker(&waker);
                // Pin the future on the stack for this poll.
                let mut pinned = unsafe { Pin::new_unchecked(&mut *fut) };
                match pinned.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        // Future completed; drop it.
                    }
                    Poll::Pending => {
                        // Put the future back and let the waker re‑queue later.
                        *future_slot = Some(fut);
                    }
                }
            }
        }
    }
}

/// Create a `Waker` for a given task.
fn task_waker(task: Arc<Task>) -> Waker {
    // The raw pointer stored in the waker is a clone of the Arc.
    let raw = Arc::into_raw(task) as *const ();
    unsafe { Waker::from_raw(raw_waker(raw)) }
}

unsafe fn raw_waker(data: *const ()) -> RawWaker {
    RawWaker::new(data, &VTABLE)
}

unsafe fn waker_clone(data: *const ()) -> RawWaker {
    // Recreate the Arc to bump the ref count.
    let arc = Arc::from_raw(data as *const Task);
    let cloned = arc.clone();
    // Forget both to keep the original ref counts.
    std::mem::forget(arc);
    std::mem::forget(cloned);
    RawWaker::new(data, &VTABLE)
}

unsafe fn waker_wake(data: *const ()) {
    // Recreate the Arc, push the task back onto the queue, then drop the Arc.
    let task = Arc::from_raw(data as *const Task);
    let queue = task.queue.clone();
    queue.lock().unwrap().push_back(task.clone());
    // Drop the temporary Arc from from_raw.
    // The cloned Arc stays alive in the queue.
}

unsafe fn waker_wake_by_ref(data: *const ()) {
    // Same as wake but without consuming the Arc.
    let task = Arc::from_raw(data as *const Task);
    let queue = task.queue.clone();
    queue.lock().unwrap().push_back(task.clone());
    // Forget the original Arc to keep the ref count unchanged.
    std::mem::forget(task);
}

unsafe fn waker_drop(data: *const ()) {
    // Decrement the ref count.
    let _ = Arc::from_raw(data as *const Task);
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake_by_ref,
    waker_drop,
);