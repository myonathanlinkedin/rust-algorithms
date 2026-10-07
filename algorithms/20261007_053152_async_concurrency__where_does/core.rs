use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A task that owns a future.
pub struct Task {
    // The future is boxed and pinned because we need to poll it later.
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + 'static>>>,
    // Reference back to the executor's queue for waking.
    pub executor_queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl Task {
    fn poll(self: Arc<Self>) {
        // Create a waker that knows how to re‑queue this task.
        let waker = task_waker(self.clone());
        let mut ctx = Context::from_waker(&waker);
        // Lock the future for exclusive access while polling.
        let mut future_guard = self.future.lock().unwrap();
        match future_guard.as_mut().poll(&mut ctx) {
            Poll::Ready(()) => {
                // Task completed; nothing to do.
            }
            Poll::Pending => {
                // The waker will re‑queue the task when it is woken.
            }
        }
    }
}

/// The executor holds a queue of ready tasks.
pub struct Executor {
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
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
            executor_queue: self.queue.clone(),
        });
        self.queue.lock().unwrap().push_back(task);
    }

    /// Run until the task queue is empty.
    pub fn run(&self) {
        while let Some(task) = self.queue.lock().unwrap().pop_front() {
            task.poll();
        }
    }
}

// ---------- Waker implementation ----------
fn task_waker(task: Arc<Task>) -> Waker {
    // SAFETY: we construct a RawWaker that only uses the provided vtable.
    unsafe { Waker::from_raw(raw_waker(task)) }
}

fn raw_waker(task: Arc<Task>) -> RawWaker {
    // Increase the ref count for the RawWaker.
    let data = Arc::into_raw(task) as *const ();
    RawWaker::new(data, &VTABLE)
}

unsafe fn waker_clone(data: *const ()) -> RawWaker {
    // Recreate the Arc to bump the ref count.
    let arc = Arc::from_raw(data as *const Task);
    let cloned = arc.clone();
    // Forget the original to keep its ref count unchanged.
    std::mem::forget(arc);
    raw_waker(cloned)
}

unsafe fn waker_wake(data: *const ()) {
    // Recreate the Arc and push the task back onto the queue.
    let task = Arc::from_raw(data as *const Task);
    task.executor_queue.lock().unwrap().push_back(task.clone());
    // Drop the temporary Arc created by from_raw.
    // The cloned Arc remains in the queue.
}

unsafe fn waker_wake_by_ref(data: *const ()) {
    // Same as wake but keep the original Arc alive.
    let task = Arc::from_raw(data as *const Task);
    task.executor_queue.lock().unwrap().push_back(task.clone());
    // Forget to avoid decreasing the ref count.
    std::mem::forget(task);
}

unsafe fn waker_drop(data: *const ()) {
    // Decrease the ref count by reconstructing the Arc.
    let _ = Arc::from_raw(data as *const Task);
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake_by_ref,
    waker_drop,
);

// ---------- Helper future that yields once ----------
/// A future that returns `Pending` on the first poll and `Ready(())` on the second.
pub struct YieldNow {
    pub yielded: bool,

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