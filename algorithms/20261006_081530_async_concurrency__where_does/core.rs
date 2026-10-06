use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

/// Represents the state of a task in the scheduler.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Suspended,
    Completed,
}

/// A unit of work managed by the scheduler.
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: usize,
    pub state: TaskState,
    pub priority: u8,
    pub execution_time: u64,

}

impl Task {
    pub fn new(id: usize, priority: u8) -> Self {
        Task {
            id,
            state: TaskState::Ready,
            priority,
            execution_time: 0,
        }
    }
}

/// The core scheduler that manages task execution.
/// In a real async runtime, this would be the "waker" or executor loop.
pub struct Scheduler {
    pub ready_queue: VecDeque<Task>,
    pub running_task: Option<Task>,
    pub completed_tasks: Vec<Task>,
    pub suspended_tasks: Vec<Task>,
    pub total_executed: AtomicUsize,

}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            ready_queue: VecDeque::new(),
            running_task: None,
            completed_tasks: Vec::new(),
            suspended_tasks: Vec::new(),
            total_executed: AtomicUsize::new(0),
        }
    }

    /// Add a new task to the ready queue.
    pub fn spawn(&mut self, task: Task) {
        self.ready_queue.push_back(task);
    }

    /// Execute the next task from the ready queue.
    /// Returns the task after execution.
    pub fn step(&mut self) -> Option<Task> {
        if self.running_task.is_some() {
            return self.running_task.clone();
        }

        // Select the highest priority task from the ready queue
        if let Some(pos) = self.ready_queue.iter().position(|t| {
            self.ready_queue.iter().all(|other| other.priority <= t.priority)
        }) {
            let task = self.ready_queue.remove(pos).unwrap();
            let mut executed_task = task.clone();
            executed_task.state = TaskState::Running;
            executed_task.execution_time += 1;
            self.running_task = Some(executed_task.clone());
            self.total_executed.fetch_add(1, Ordering::SeqCst);
            Some(executed_task)
        } else {
            None
        }
    }

    /// Complete the currently running task.
    pub fn complete_current(&mut self) -> Option<Task> {
        if let Some(mut task) = self.running_task.take() {
            task.state = TaskState::Completed;
            self.completed_tasks.push(task.clone());
            Some(task)
        } else {
            None
        }
    }

    /// Suspend the currently running task.
    pub fn suspend_current(&mut self) -> Option<Task> {
        if let Some(mut task) = self.running_task.take() {
            task.state = TaskState::Suspended;
            self.suspended_tasks.push(task.clone());
            Some(task)
        } else {
            None
        }
    }

    /// Resume a suspended task by ID.
    pub fn resume(&mut self, task_id: usize) -> Option<Task> {
        if let Some(pos) = self.suspended_tasks.iter().position(|t| t.id == task_id) {
            let mut task = self.suspended_tasks.remove(pos);
            task.state = TaskState::Ready;
            self.ready_queue.push_back(task.clone());
            Some(task)
        } else {
            None
        }
    }

    /// Get the number of tasks in the ready queue.
    pub fn ready_count(&self) -> usize {
        self.ready_queue.len()
    }

    /// Get the number of completed tasks.
    pub fn completed_count(&self) -> usize {
        self.completed_tasks.len()
    }

    /// Get the number of suspended tasks.
    pub fn suspended_count(&self) -> usize {
        self.suspended_tasks.len()
    }

    /// Get the total number of execution steps performed.
    pub fn total_executed(&self) -> usize {
        self.total_executed.load(Ordering::SeqCst)
    }

    /// Check if the scheduler is idle (no ready or running tasks).
    pub fn is_idle(&self) -> bool {
        self.ready_queue.is_empty() && self.running_task.is_none()
    }

    /// Run the scheduler until all tasks are completed.
    pub fn run_until_complete(&mut self) -> Vec<Task> {
        let mut results = Vec::new();
        while !self.is_idle() {
            if self.running_task.is_none() {
                if self.step().is_none() {
                    break;
                }
            }
            if let Some(task) = self.complete_current() {
                results.push(task);
            }
        }
        results
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
