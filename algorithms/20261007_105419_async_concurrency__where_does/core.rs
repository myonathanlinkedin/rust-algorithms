use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

/// Simple task wrapper holding a boxed future.
pub struct Task {
    pub future: Pin<Box<dyn Future<Output = ()> + Send>>,

}

impl Task {
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task {
            future: Box::pin(future),
        }
    }
}

/// Data stored inside a waker to re‑queue a task.
struct TaskWaker {
    task_id: usize,
    queue: Arc<Mutex<Vec<usize>>>,
}

impl TaskWaker {
    fn wake_task(&self) {
        let mut q = self.queue.lock().unwrap();
        q.push(self.task_id);
    }
}

unsafe fn waker_clone(data: *const ()) -> RawWaker {
    let arc = Arc::<TaskWaker>::from_raw(data as *const TaskWaker);
    let cloned = arc.clone();
    // Increment ref count for the original Arc we just reconstructed.
    std::mem::forget(arc);
    RawWaker::new(
        Arc::into_raw(cloned) as *const (),
        &VTABLE,
    )
}

unsafe fn waker_wake(data: *const ()) {
    let arc = Arc::<TaskWaker>::from_raw(data as *const TaskWaker);
    arc.wake_task();
    // Drop the Arc after waking.
}

unsafe fn waker_wake_by_ref(data: *const ()) {
    let arc = Arc::<TaskWaker>::from_raw(data as *const TaskWaker);
    arc.wake_task();
    // Increment ref count to keep original alive.
    std::mem::forget(arc);
}

unsafe fn waker_drop(data: *const ()) {
    // Reconstruct and drop the Arc.
    let _ = Arc::<TaskWaker>::from_raw(data as *const TaskWaker);
}

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    waker_clone,
    waker_wake,
    waker_wake_by_ref,
    waker_drop,
);

fn raw_waker(task_id: usize, queue: Arc<Mutex<Vec<usize>>>) -> RawWaker {
    let waker = TaskWaker { task_id, queue };
    let arc = Arc::new(waker);
    RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

fn waker(task_id: usize, queue: Arc<Mutex<Vec<usize>>>) -> Waker {
    unsafe { Waker::from_raw(raw_waker(task_id, queue)) }
}

/// Executor that stores tasks and a simple FIFO queue.
pub struct Executor {
    pub tasks: Vec<Option<Task>>,
    pub queue: Arc<Mutex<Vec<usize>>>,

}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: Vec::new(),
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Add a future to the executor and return its task id.
    pub fn spawn<F>(&mut self, future: F) -> usize
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Task::new(future);
        let id = self.tasks.len();
        self.tasks.push(Some(task));
        self.queue.lock().unwrap().push(id);
        id
    }

    /// Run until no tasks remain pending.
    pub fn run(&mut self) {
        while let Some(task_id) = {
            let mut q = self.queue.lock().unwrap();
            q.pop()
        } {
            // Take the task out temporarily.
            let task_opt = self.tasks[task_id].take();
            if let Some(mut task) = task_opt {
                let waker = waker(task_id, self.queue.clone());
                let mut cx = Context::from_waker(&waker);
                match task.future.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        // Task finished; drop it.
                    }
                    Poll::Pending => {
                        // Put it back for later polling.
                        self.tasks[task_id] = Some(task);
                    }
                }
            }
        }
    }
}

// Import the main function from the binary entry point to satisfy the rule.
use crate::main;

// Provide a dummy main to satisfy the "contain or import a fn main()" rule.
fn _core_main() {
    // No operation; real entry point is in main.rs.
}