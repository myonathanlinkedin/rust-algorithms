use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// Simple future that yields once before completing.
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

/// Internal shared state for the executor.
struct ExecutorInner {
    /// Queue of task ids ready to be polled.
    queue: VecDeque<usize>,
    /// Storage for tasks; None indicates completed.
    tasks: Vec<Option<Pin<Box<dyn Future<Output = ()> + Send>>>>,
}

impl ExecutorInner {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            tasks: Vec::new(),
        }
    }
}

/// A lightweight single‑threaded async executor.
pub struct Executor {
    pub inner: Arc<Mutex<ExecutorInner>>,

}

impl Executor {
    /// Create a new executor.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(ExecutorInner::new())),
        }
    }

    /// Spawn a future onto the executor. Returns a task identifier.
    pub fn spawn<F>(&self, fut: F) -> usize
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let mut inner = self.inner.lock().unwrap();
        let task_id = inner.tasks.len();
        inner.tasks.push(Some(Box::pin(fut)));
        inner.queue.push_back(task_id);
        task_id
    }

    /// Run the executor until all tasks have completed.
    pub fn run(&self) {
        loop {
            let task_id_opt = {
                let mut inner = self.inner.lock().unwrap();
                inner.queue.pop_front()
            };
            let task_id = match task_id_opt {
                Some(id) => id,
                None => break,
            };

            // Take the future out temporarily to avoid double‑borrow.
            let mut future_opt = {
                let mut inner = self.inner.lock().unwrap();
                inner.tasks[task_id].take()
            };

            if let Some(mut future) = future_opt.take() {
                // Build a waker that can re‑queue this task.
                let waker = self.task_waker(task_id);
                let mut cx = Context::from_waker(&waker);
                match future.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        // Task finished; drop it.
                        let mut inner = self.inner.lock().unwrap();
                        inner.tasks[task_id] = None;
                    }
                    Poll::Pending => {
                        // Put the future back and continue.
                        let mut inner = self.inner.lock().unwrap();
                        inner.tasks[task_id] = Some(future);
                    }
                }
            }
        }
    }

    /// Construct a waker that, when woken, pushes the task id back onto the queue.
    fn task_waker(&self, task_id: usize) -> Waker {
        // Clone the Arc to keep it alive in the waker.
        let inner = Arc::clone(&self.inner);
        // Allocate the waker data on the heap.
        let data = Arc::new(TaskWakerData { inner, task_id });

        unsafe { Waker::from_raw(Self::raw_waker(data)) }
    }

    unsafe fn raw_waker(data: Arc<TaskWakerData>) -> RawWaker {
        // Increase the ref count for the RawWaker.
        let ptr = Arc::into_raw(data) as *const ();
        RawWaker::new(
            ptr,
            &RawWakerVTable::new(
                Self::waker_clone,
                Self::waker_wake,
                Self::waker_wake_by_ref,
                Self::waker_drop,
            ),
        )
    }

    unsafe fn waker_clone(ptr: *const ()) -> RawWaker {
        // Recreate the Arc to bump the ref count.
        let data = Arc::from_raw(ptr as *const TaskWakerData);
        let cloned = Arc::clone(&data);
        // Forget the original to keep its count unchanged.
        std::mem::forget(data);
        Self::raw_waker(cloned)
    }

    unsafe fn waker_wake(ptr: *const ()) {
        // Take ownership and wake.
        let data = Arc::from_raw(ptr as *const TaskWakerData);
        data.wake();
        // Dropping `data` decrements the ref count.
    }

    unsafe fn waker_wake_by_ref(ptr: *const ()) {
        // Clone to keep the original alive.
        let data = Arc::from_raw(ptr as *const TaskWakerData);
        data.wake();
        // Increment ref count again because we didn't take ownership.
        std::mem::forget(data);
    }

    unsafe fn waker_drop(ptr: *const ()) {
        // Decrement the ref count.
        let _ = Arc::from_raw(ptr as *const TaskWakerData);
    }
}
struct TaskWakerData {
    inner: Arc<Mutex<ExecutorInner>>,
    task_id: usize,
}

impl TaskWakerData {
    fn wake(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.queue.push_back(self.task_id);
    }
}