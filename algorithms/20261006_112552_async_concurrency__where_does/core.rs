use std::future::Future;
use std::pin::Pin;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A simple future that yields once before completing.
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

/// A task wraps a future that can be scheduled.
pub struct Task {
    // The future is boxed and protected by a mutex because the executor may
    // access it from multiple threads (though this simple executor runs single‑threaded).
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,

}

impl Task {
    fn new<F>(future: F) -> Arc<Self>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Arc::new(Self {
            future: Mutex::new(Box::pin(future)),
        })
    }
}

/// Simple single‑threaded executor with its own task queue.
pub struct SimpleExecutor {
    pub queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl SimpleExecutor {
    /// Create a new executor.
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Spawn a future onto the executor.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Task::new(future);
        self.enqueue(task);
    }

    /// Run tasks until the queue is empty.
    pub fn run(&mut self) {
        while let Some(task) = self.dequeue() {
            // Create a waker that can re‑queue the task if it returns Pending.
            let waker = task_waker(task.clone(), self.queue.clone());
            let mut ctx = Context::from_waker(&waker);
            let mut future_guard = task.future.lock().unwrap();
            match future_guard.as_mut().poll(&mut ctx) {
                Poll::Ready(()) => {
                    // Task completed; drop it.
                }
                Poll::Pending => {
                    // The waker will have re‑queued the task if needed.
                }
            }
        }
    }

    fn enqueue(&self, task: Arc<Task>) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(task);
    }

    fn dequeue(&self) -> Option<Arc<Task>> {
        let mut q = self.queue.lock().unwrap();
        q.pop_front()
    }
}

// Helper to build a Waker that pushes the task back onto the executor's queue.
fn task_waker(task: Arc<Task>, queue: Arc<Mutex<VecDeque<Arc<Task>>>>) -> Waker {
    // The raw waker data is a clone of the Arc<Task> plus the queue.
    // We store both in a single Arc to keep the pointer size constant.
    let data = Arc::new(WakerData { task, queue });
    unsafe { Waker::from_raw(raw_waker(data)) }
}
struct WakerData {
    task: Arc<Task>,
    queue: Arc<Mutex<VecDeque<Arc<Task>>>>,
}

// Construct a RawWaker from an Arc<WakerData>.
unsafe fn raw_waker(data: Arc<WakerData>) -> RawWaker {
    // Increase the ref count for the RawWaker.
    let ptr = Arc::into_raw(data) as *const ();
    RawWaker::new(ptr, &VTABLE)
}

// VTable functions for the custom waker.
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_waker,
    wake_waker,
    wake_by_ref_waker,
    drop_waker,
);

unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
    // Recreate the Arc to bump the ref count.
    let arc = Arc::from_raw(ptr as *const WakerData);
    let cloned = arc.clone();
    // Forget the original to keep its count unchanged.
    std::mem::forget(arc);
    raw_waker(cloned)
}

unsafe fn wake_waker(ptr: *const ()) {
    // Convert back to Arc and then drop after waking.
    let arc = Arc::from_raw(ptr as *const WakerData);
    enqueue_task(&arc);
    // Drop the Arc (decrement ref count).
}

unsafe fn wake_by_ref_waker(ptr: *const ()) {
    // Clone the Arc to keep the original alive.
    let arc = Arc::from_raw(ptr as *const WakerData);
    enqueue_task(&arc);
    // Increment ref count again because we called from_raw.
    std::mem::forget(arc);
}

unsafe fn drop_waker(ptr: *const ()) {
    // Recreate the Arc to decrement the count.
    let _ = Arc::from_raw(ptr as *const WakerData);
}

// Enqueue the task using the stored queue.
fn enqueue_task(data: &Arc<WakerData>) {
    let task = data.task.clone();
    let mut q = data.queue.lock().unwrap();
    q.push_back(task);
}