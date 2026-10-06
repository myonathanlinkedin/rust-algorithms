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
        Self { yielded: false }
    }
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

struct ExecutorInner {
    queue: Mutex<VecDeque<Arc<Task>>>,
}
pub struct Executor {
    pub inner: Arc<ExecutorInner>,

}

impl Executor {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(ExecutorInner {
                queue: Mutex::new(VecDeque::new()),
            }),
        }
    }

    /// Spawn a future that runs to completion. The future must be `Send` and `'static`.
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Some(Box::pin(fut))),
            executor: self.inner.clone(),
        });
        self.schedule(task);
    }

    /// Run the executor until no tasks remain.
    pub fn run(&self) {
        while let Some(task) = self.inner.queue.lock().unwrap().pop_front() {
            let waker = task_waker(&task);
            let mut ctx = Context::from_waker(&waker);
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut fut) = future_slot.take() {
                match fut.as_mut().poll(&mut ctx) {
                    Poll::Pending => {
                        // Put the future back and let the waker re‑schedule it.
                        *future_slot = Some(fut);
                    }
                    Poll::Ready(()) => {
                        // Future completed; drop it.
                    }
                }
            }
        }
    }

    fn schedule(&self, task: Arc<Task>) {
        let mut q = self.inner.queue.lock().unwrap();
        q.push_back(task);
    }
}

struct Task {
    future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
    executor: Arc<ExecutorInner>,
}

// ---------- Waker implementation ----------
fn task_waker(task: &Arc<Task>) -> Waker {
    // Increase ref count for the raw pointer.
    let raw = Arc::into_raw(task.clone()) as *const ();
    unsafe { Waker::from_raw(raw_waker(raw)) }
}

fn raw_waker(data: *const ()) -> RawWaker {
    RawWaker::new(data, &VTABLE)
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    raw_waker_clone,
    raw_waker_wake,
    raw_waker_wake_by_ref,
    raw_waker_drop,
);

unsafe fn raw_waker_clone(data: *const ()) -> RawWaker {
    // Recreate Arc to bump ref count.
    let arc = Arc::from_raw(data as *const Task);
    let cloned = arc.clone();
    // Forget the temporary Arc to keep original alive.
    std::mem::forget(arc);
    RawWaker::new(Arc::into_raw(cloned) as *const (), &VTABLE)
}

unsafe fn raw_waker_wake(data: *const ()) {
    // Take ownership, schedule, then drop.
    let task = Arc::from_raw(data as *const Task);
    task.executor
        .queue
        .lock()
        .unwrap()
        .push_back(task.clone());
    // Dropping `task` here decrements the ref count.
}

unsafe fn raw_waker_wake_by_ref(data: *const ()) {
    // Clone to schedule without consuming the original.
    let task = Arc::from_raw(data as *const Task);
    task.executor
        .queue
        .lock()
        .unwrap()
        .push_back(task.clone());
    // Keep original alive.
    std::mem::forget(task);
}

unsafe fn raw_waker_drop(data: *const ()) {
    // Decrement ref count.
    let _ = Arc::from_raw(data as *const Task);
}

// === END OF core.rs ===