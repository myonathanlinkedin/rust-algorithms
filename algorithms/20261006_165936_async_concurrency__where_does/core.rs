use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A simple future that yields once before completing.
pub struct YieldNow {
    pub yielded: bool,

}

impl YieldNow {
    pub fn new() -> Self {
        YieldNow { yielded: false }
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

/// Wrapper around a boxed future that can be polled repeatedly.
pub struct Task {
    // The future is protected by a mutex because the waker may access it from
    // the executor's wake path.
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + 'static>>>,

}

impl Task {
    pub fn new<F>(future: F) -> Arc<Self>
    where
        F: Future<Output = ()> + 'static,
    {
        Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
        })
    }
}

/// Inner shared state of the executor.
struct ExecutorInner {
    queue: Mutex<VecDeque<Arc<Task>>>,
}

impl ExecutorInner {
    fn push_task(&self, task: Arc<Task>) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(task);
    }

    fn pop_task(&self) -> Option<Arc<Task>> {
        let mut q = self.queue.lock().unwrap();
        q.pop_front()
    }
}

/// A very small single‑threaded executor.
pub struct Executor {
    pub inner: Arc<ExecutorInner>,

}

impl Executor {
    pub fn new() -> Self {
        Executor {
            inner: Arc::new(ExecutorInner {
                queue: Mutex::new(VecDeque::new()),
            }),
        }
    }

    /// Spawn a future onto the executor.
    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let task = Task::new(future);
        self.inner.push_task(task);
    }

    /// Run until no tasks remain.
    pub fn run(&self) {
        while let Some(task) = self.inner.pop_task() {
            // Create a waker that can re‑queue the task.
            let waker = task_waker(task.clone(), self.inner.clone());
            let mut ctx = Context::from_waker(&waker);
            let mut future_guard = task.future.lock().unwrap();
            let poll = future_guard.as_mut().poll(&mut ctx);
            drop(future_guard);
            if let Poll::Pending = poll {
                // The task will be re‑queued by its waker.
            }
        }
    }
}

// ---------- Waker implementation ----------
fn task_waker(task: Arc<Task>, exec: Arc<ExecutorInner>) -> Waker {
    // The raw waker holds a clone of the task and the executor.
    let data = Arc::into_raw(Arc::new(WakerData { task, exec })) as *const ();
    unsafe { Waker::from_raw(raw_waker(data)) }
}

struct WakerData {
    task: Arc<Task>,
    exec: Arc<ExecutorInner>,
}

unsafe fn raw_waker(data: *const ()) -> RawWaker {
    RawWaker::new(
        data,
        &RawWakerVTable::new(
            raw_waker_clone,
            raw_waker_wake,
            raw_waker_wake_by_ref,
            raw_waker_drop,
        ),
    )
}

unsafe fn raw_waker_clone(data: *const ()) -> RawWaker {
    // Increase the Arc count.
    let arc = Arc::from_raw(data as *const WakerData);
    let _ = arc.clone(); // bump refcount
    let ptr = Arc::into_raw(arc);
    raw_waker(ptr as *const ())
}

unsafe fn raw_waker_wake(data: *const ()) {
    // Take ownership and wake once.
    let arc = Arc::from_raw(data as *const WakerData);
    arc.exec.push_task(arc.task.clone());
    // Drop the Arc (decrement refcount).
}

unsafe fn raw_waker_wake_by_ref(data: *const ()) {
    // Clone the Arc to keep it alive after waking.
    let arc = Arc::from_raw(data as *const WakerData);
    arc.exec.push_task(arc.task.clone());
    // Don't drop the original Arc; just increase refcount back.
    let _ = Arc::into_raw(arc);
}

unsafe fn raw_waker_drop(data: *const ()) {
    // Decrement the Arc count.
    let _ = Arc::from_raw(data as *const WakerData);
}

// ---------- Helper async function ----------
/// An async function that yields once before completing.
pub async fn async_yield() {
    YieldNow::new().await;
}

// === END OF core.rs ===