use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A task that owns a boxed future.
pub struct Task {
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,

}

impl Task {
    fn new<F>(future: F) -> Arc<Self>
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
        })
    }
}

/// Shared queue type used by the executor and wakers.
type TaskQueue = Arc<Mutex<VecDeque<Arc<Task>>>>;

/// Data stored inside a custom waker.
struct WakerData {
    task: Arc<Task>,
    queue: TaskQueue,
}

// Raw waker vtable implementation.
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake_by_ref,
    waker_drop,
);

unsafe fn waker_clone(data: *const ()) -> RawWaker {
    let arc = Arc::from_raw(data as *const WakerData);
    let cloned = arc.clone();
    // Prevent dropping the original Arc.
    let _ = Arc::into_raw(arc);
    RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
}

unsafe fn waker_wake(data: *const ()) {
    let waker_data = Arc::from_raw(data as *const WakerData);
    let mut queue = waker_data.queue.lock().unwrap();
    queue.push_back(waker_data.task.clone());
    // Arc is dropped here, decreasing ref count.
}

unsafe fn waker_wake_by_ref(data: *const ()) {
    let waker_data = Arc::from_raw(data as *const WakerData);
    {
        let mut queue = waker_data.queue.lock().unwrap();
        queue.push_back(waker_data.task.clone());
    }
    // Keep the Arc alive for the caller.
    let _ = Arc::into_raw(waker_data);
}

unsafe fn waker_drop(data: *const ()) {
    let _ = Arc::from_raw(data as *const WakerData);
}

/// Construct a `Waker` that re‑queues the associated task.
fn task_waker(task: Arc<Task>, queue: TaskQueue) -> Waker {
    let data = Arc::new(WakerData { task, queue });
    let raw = RawWaker::new(Arc::into_raw(data) as *const (), &VTABLE);
    unsafe { Waker::from_raw(raw) }
}

/// Simple single‑threaded executor.
pub struct Executor {
    pub queue: TaskQueue,

}

impl Executor {
    /// Create a new empty executor.
    pub fn new() -> Self {
        Executor {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Spawn a future onto the executor.
    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Task::new(future);
        self.queue.lock().unwrap().push_back(task);
    }

    /// Run tasks until the queue is empty.
    pub fn run(&self) {
        while let Some(task) = self.queue.lock().unwrap().pop_front() {
            let waker = task_waker(task.clone(), self.queue.clone());
            let mut cx = Context::from_waker(&waker);
            let mut future_guard = task.future.lock().unwrap();
            if let Poll::Pending = future_guard.as_mut().poll(&mut cx) {
                // The task will be re‑queued by its waker.
            }
        }
    }
}

/// A future that yields once, allowing the executor to schedule other tasks.
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

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if !self.yielded {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

/// Run a future to completion using the `Executor`.
pub fn block_on<F>(future: F) -> F::Output
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    use std::sync::mpsc;
    let (tx, rx) = mpsc::channel();

    let exec = Executor::new();
    exec.spawn(async move {
        let res = future.await;
        let _ = tx.send(res);
    });
    exec.run();
    rx.recv().expect("Executor failed to send result")
}