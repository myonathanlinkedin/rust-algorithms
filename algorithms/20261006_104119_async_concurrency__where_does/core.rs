use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};

/// Simple future that yields once before completing.
#[derive(Debug, Clone, PartialEq)]
pub struct YieldOnce {
    pub yielded: bool,

}

impl YieldOnce {
    pub fn new() -> Self {
        Self { yielded: false }
    }
}

impl Future for YieldOnce {
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

/// Waker implementation that pushes the associated task id back onto the executor's queue.
#[derive(Debug)]
struct TaskWaker {
    queue: Arc<Mutex<Vec<usize>>>,
    task_id: usize,
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        let mut q = self.queue.lock().unwrap();
        q.push(self.task_id);
    }

    fn wake_by_ref(self: &Arc<Self>) {
        let mut q = self.queue.lock().unwrap();
        q.push(self.task_id);
    }
}

/// Minimal single‑threaded async executor.
pub struct Executor {
    /// Storage for all spawned tasks; `None` indicates a completed task.
    pub tasks: Vec<Option<Pin<Box<dyn Future<Output = ()> + 'static>>>>,
    /// Queue of task indices ready to be polled.
    pub queue: Arc<Mutex<Vec<usize>>>,

}

impl Executor {
    /// Create a new empty executor.
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Spawn a future onto the executor. Returns the internal task id.
    pub fn spawn<F>(&mut self, fut: F) -> usize
    where
        F: Future<Output = ()> + 'static,
    {
        let task_id = self.tasks.len();
        self.tasks.push(Some(Box::pin(fut)));
        self.queue.lock().unwrap().push(task_id);
        task_id
    }

    /// Run the executor until no tasks remain.
    pub fn run(&mut self) {
        while let Some(task_id) = self.next_ready_task() {
            // Take the future out temporarily to satisfy the borrow checker.
            let mut task_opt = self.tasks[task_id].take();
            if let Some(mut fut) = task_opt {
                // Build a waker that can re‑queue this task.
                let waker = {
                    let w = TaskWaker {
                        queue: Arc::clone(&self.queue),
                        task_id,
                    };
                    Waker::from(Arc::new(w))
                };
                let mut cx = Context::from_waker(&waker);
                match fut.as_mut().poll(&mut cx) {
                    Poll::Ready(()) => {
                        // Task completed; drop it.
                        self.tasks[task_id] = None;
                    }
                    Poll::Pending => {
                        // Put the future back; it will be re‑queued by the waker.
                        self.tasks[task_id] = Some(fut);
                    }
                }
            }
        }
    }

    /// Retrieve the next task id from the ready queue, if any.
    fn next_ready_task(&self) -> Option<usize> {
        let mut q = self.queue.lock().unwrap();
        q.pop()
    }
}