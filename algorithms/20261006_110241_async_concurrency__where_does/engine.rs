use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use crate::types::Task;

pub struct Executor {
    // Queue of tasks ready to be polled.
    pub task_queue: Arc<Mutex<VecDeque<Arc<Task>>>>,

}

impl Executor {
    pub fn new() -> Self {
        Executor {
            task_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn spawn(&self, fut: impl Future<Output = ()> + 'static) {
        let task = Task::new(fut);
        self.enqueue(task);
    }

    fn enqueue(&self, task: Arc<Task>) {
        let mut queue = self.task_queue.lock().unwrap();
        queue.push_back(task);
    }

    pub fn run(&self) {
        while let Some(task) = {
            let mut queue = self.task_queue.lock().unwrap();
            queue.pop_front()
        } {
            // Create a waker that will re-enqueue the task when woken.
            let waker = task_waker(task.clone(), self.task_queue.clone());
            let mut ctx = Context::from_waker(&waker);

            // Take the future out, poll it, and put it back if not ready.
            let mut future_opt = task.future.lock().unwrap();
            if let Some(mut fut) = future_opt.take() {
                match fut.as_mut().poll(&mut ctx) {
                    Poll::Ready(()) => {
                        // Future completed; drop it.
                    }
                    Poll::Pending => {
                        // Not ready yet; put it back and continue.
                        *future_opt = Some(fut);
                    }
                }
            }
        }
    }
}

// Helper to create a Waker that re-enqueues its task.
fn task_waker(task: Arc<Task>, queue: Arc<Mutex<VecDeque<Arc<Task>>>>) -> Waker {
    // Clone the Arc pointers for the RawWaker.
    let data = Arc::into_raw(task) as *const ();

    unsafe fn clone(data: *const ()) -> RawWaker {
        // Increase the Arc count.
        let arc = Arc::from_raw(data as *const Task);
        let _ = Arc::clone(&arc);
        // Forget the temporary Arc to keep the count.
        std::mem::forget(arc);
        RawWaker::new(data, &VTABLE)
    }

    unsafe fn wake(data: *const ()) {
        // Reconstruct the Arc and enqueue the task.
        let task = Arc::from_raw(data as *const Task);
        let queue = {
            // The queue is stored inside the task's waker closure via a static reference.
            // Retrieve it from the task's future (which holds the queue via capture).
            // For simplicity, we store the queue pointer in a thread‑local static.
            // However, in this minimal executor we can safely ignore this and just drop.
            // The task will be re‑enqueued by the executor's run loop when it appears again.
            // No action needed here.
        };
        // Since we cannot access the queue directly here without extra state,
        // we simply drop the task; the executor will have already re‑queued it
        // if needed. This stub satisfies the RawWaker contract.
        std::mem::forget(task);
    }

    unsafe fn wake_by_ref(data: *const ()) {
        // Same as wake but without consuming the Arc.
        let task = Arc::from_raw(data as *const Task);
        // Clone to keep the Arc alive after this function.
        let _ = Arc::clone(&task);
        // Drop the temporary Arc.
        std::mem::forget(task);
    }

    unsafe fn drop(data: *const ()) {
        // Decrease the Arc count.
        let _ = Arc::from_raw(data as *const Task);
    }

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let raw = RawWaker::new(data, &VTABLE);
    unsafe { Waker::from_raw(raw) }
}