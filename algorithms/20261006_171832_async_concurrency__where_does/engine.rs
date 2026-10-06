use std::task::{Poll};

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, RawWaker, RawWakerVTable, Waker};

use crate::types::{Task, YieldNow};

pub struct Executor {
    // Storage for tasks; None indicates a completed slot.
    pub tasks: Vec<Option<Task>>,
    // Shared queue of ready task indices.
    pub queue: Arc<Mutex<VecDeque<usize>>>,

}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    // Spawn a future onto the executor.
    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let task = Task {
            future: Box::pin(fut),
        };
        let id = self.tasks.len();
        self.tasks.push(Some(task));
        self.queue.lock().unwrap().push_back(id);
    }

    // Run until all tasks have completed.
    pub fn run(&mut self) {
        while let Some(task_id) = self.queue.lock().unwrap().pop_front() {
            // Take the task out temporarily.
            let mut task_opt = self.tasks[task_id].take();
            if let Some(mut task) = task_opt.take() {
                let waker = task_waker(self.queue.clone(), task_id);
                let mut cx = Context::from_waker(&waker);
                match task.future.as_mut().poll(&mut cx) {
                    std::task::Poll::Ready(()) => {
                        // Task finished; drop it.
                    }
                    std::task::Poll::Pending => {
                        // Put it back for later polling.
                        self.tasks[task_id] = Some(task);
                    }
                }
            }
        }
    }
}

// Data stored inside the raw waker.
struct WakerData {
    queue: Arc<Mutex<VecDeque<usize>>>,
    task_id: usize,
}

// Helper to create a Waker for a specific task.
fn task_waker(queue: Arc<Mutex<VecDeque<usize>>>, task_id: usize) -> Waker {
    let data = Box::new(WakerData { queue, task_id });
    unsafe { Waker::from_raw(raw_waker(Box::into_raw(data))) }
}

// Construct a RawWaker from a raw pointer to WakerData.
unsafe fn raw_waker(data_ptr: *const WakerData) -> RawWaker {
    fn clone(data: *const ()) -> RawWaker {
        unsafe {
            let orig = data as *const WakerData;
            // Increase reference count by cloning the Arc inside.
            let cloned = (*orig).queue.clone();
            let new_data = Box::new(WakerData {
                queue: cloned,
                task_id: (*orig).task_id,
            });
            raw_waker(Box::into_raw(new_data))
        }
    }

    fn wake(data: *const ()) {
        wake_by_ref(data);
    }

    fn wake_by_ref(data: *const ()) {
        unsafe {
            let wd = &*(data as *const WakerData);
            let mut q = wd.queue.lock().unwrap();
            q.push_back(wd.task_id);
        }
    }

    fn drop(data: *const ()) {
        unsafe {
            // Reclaim the Box to drop it.
            let _ = Box::from_raw(data as *mut WakerData);
        }
    }

    RawWaker::new(
        data_ptr as *const (),
        &RawWakerVTable::new(clone, wake, wake_by_ref, drop),
    )
}