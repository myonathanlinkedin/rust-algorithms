use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::collections::VecDeque;

/// A task that can be scheduled by the executor.
pub struct Task {
    /// The future that this task will drive to completion.
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    /// Reference to the executor's queue for rescheduling.
    pub queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl Task {
    fn poll(self: Arc<Self>) -> bool {
        // Create a waker that will re‑enqueue this task when woken.
        let waker = task_waker(self.clone());
        let mut ctx = Context::from_waker(&waker);
        let mut future_guard = self.future.lock().unwrap();
        match future_guard.as_mut().poll(&mut ctx) {
            Poll::Ready(()) => true,
            Poll::Pending => false,
        }
    }

    fn schedule(self: &Arc<Self>) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(self.clone());
    }
}

/// Simple single‑threaded executor.
pub struct SimpleExecutor {
    pub queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl SimpleExecutor {
    /// Create a new executor with an empty task queue.
    pub fn new() -> Self {
        SimpleExecutor {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    /// Spawn a future onto the executor. The future must resolve to `()`.
    pub fn spawn<F>(&self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(fut)),
            queue: self.queue.clone(),
        });
        let mut q = self.queue.lock().unwrap();
        q.push_back(task);
    }

    /// Run the executor until no tasks remain.
    pub fn run(&self) {
        loop {
            let task_opt = {
                let mut q = self.queue.lock().unwrap();
                q.pop_front()
            };
            match task_opt {
                Some(task) => {
                    if !task.poll() {
                        // Not finished; it will be rescheduled by its waker.
                    }
                }
                None => break,
            }
        }
    }
}

// ---------- Waker implementation ----------
fn task_waker(task: Arc<Task>) -> Waker {
    unsafe { Waker::from_raw(raw_waker(task)) }
}

fn raw_waker(task: Arc<Task>) -> RawWaker {
    // Convert the Arc into a raw pointer we can store in the RawWaker.
    let data = Arc::into_raw(task) as *const ();
    RawWaker::new(data, &VTABLE)
}

unsafe fn clone_raw(data: *const ()) -> RawWaker {
    // Recreate the Arc to bump the ref count, then forget it so we don't drop.
    let arc = Arc::<Task>::from_raw(data as *const Task);
    let _ = arc.clone(); // bump ref count
    std::mem::forget(arc);
    RawWaker::new(data, &VTABLE)
}

unsafe fn wake_raw(data: *const ()) {
    // Recreate the Arc, schedule the task, then drop the Arc.
    let arc = Arc::<Task>::from_raw(data as *const Task);
    arc.schedule();
    // Drop the Arc we just created (decrements ref count).
}

unsafe fn wake_by_ref_raw(data: *const ()) {
    // Recreate the Arc without taking ownership, schedule, then forget.
    let arc = Arc::<Task>::from_raw(data as *const Task);
    arc.schedule();
    std::mem::forget(arc);
}

unsafe fn drop_raw(data: *const ()) {
    // Recreate the Arc to decrement the ref count.
    let _ = Arc::<Task>::from_raw(data as *const Task);
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_raw,
    wake_raw,
    wake_by_ref_raw,
    drop_raw,
);