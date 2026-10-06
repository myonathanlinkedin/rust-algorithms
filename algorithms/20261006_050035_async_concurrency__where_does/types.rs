use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Represents the state of a task within the scheduler.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Suspended,
    Completed,
    Failed,
}

impl fmt::Display for TaskState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskState::Ready => write!(f, "Ready"),
            TaskState::Running => write!(f, "Running"),
            TaskState::Suspended => write!(f, "Suspended"),
            TaskState::Completed => write!(f, "Completed"),
            TaskState::Failed => write!(f, "Failed"),
        }
    }
}

/// Represents a single unit of work (task) in the scheduler.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub state: TaskState,
    pub priority: u8,
    pub execution_time: u64,
    pub remaining_time: u64,

}

impl Task {
    pub fn new(id: usize, name: &str, priority: u8, execution_time: u64) -> Self {
        Task {
            id,
            name: name.to_string(),
            state: TaskState::Ready,
            priority,
            execution_time,
            remaining_time: execution_time,
        }
    }

    pub fn is_complete(&self) -> bool {
        self.state == TaskState::Completed || self.state == TaskState::Failed
    }

    pub fn is_active(&self) -> bool {
        self.state == TaskState::Ready || self.state == TaskState::Running
    }
}

/// Represents the configuration for the scheduler.
#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub max_concurrency: usize,
    pub time_slice: u64,
    pub use_priority: bool,

}

impl Default for SchedulerConfig {
    fn default() -> Self {
        SchedulerConfig {
            max_concurrency: 4,
            time_slice: 10,
            use_priority: true,
        }
    }
}

/// Represents the result of a scheduling decision.
#[derive(Debug, Clone)]
pub struct SchedulingDecision {
    pub task_id: usize,
    pub time_slice: u64,
    pub reason: String,

}

/// Represents the overall state of the scheduler.
#[derive(Debug)]
pub struct SchedulerState {
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub ready_tasks: usize,
    pub running_tasks: usize,
    pub suspended_tasks: usize,
    pub total_time_elapsed: u64,
    pub context_switches: usize,

}

impl SchedulerState {
    pub fn new() -> Self {
        SchedulerState {
            total_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
            ready_tasks: 0,
            running_tasks: 0,
            suspended_tasks: 0,
            total_time_elapsed: 0,
            context_switches: 0,
        }
    }
}

/// Represents a thread pool worker.
#[derive(Debug)]
pub struct Worker {
    pub id: usize,
    pub current_task: Option<usize>,
    pub tasks_completed: usize,
    pub is_active: bool,

}

impl Worker {
    pub fn new(id: usize) -> Self {
        Worker {
            id,
            current_task: None,
            tasks_completed: 0,
            is_active: false,
        }
    }
}

/// Represents the thread pool.
#[derive(Debug)]
pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub active_workers: usize,

}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let workers = (0..size).map(Worker::new).collect();
        ThreadPool {
            workers,
            active_workers: 0,
        }
    }

    pub fn get_worker(&self, id: usize) -> Option<&Worker> {
        self.workers.get(id)
    }

    pub fn get_active_worker_count(&self) -> usize {
        self.active_workers
    }
}

/// Represents the scheduler itself.
#[derive(Debug)]
pub struct Scheduler {
    pub config: SchedulerConfig,
    pub tasks: Vec<Task>,
    pub thread_pool: ThreadPool,
    pub state: SchedulerState,
    pub current_time: u64,
    pub ready_queue: Vec<usize>,
    pub running_queue: Vec<usize>,
    pub completed_queue: Vec<usize>,
    pub failed_queue: Vec<usize>,

}

impl Scheduler {
    pub fn new(config: SchedulerConfig) -> Self {
        let pool_size = config.max_concurrency;
        Scheduler {
            config,
            tasks: Vec::new(),
            thread_pool: ThreadPool::new(pool_size),
            state: SchedulerState::new(),
            current_time: 0,
            ready_queue: Vec::new(),
            running_queue: Vec::new(),
            completed_queue: Vec::new(),
            failed_queue: Vec::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        let task_id = self.tasks.len();
        self.tasks.push(task);
        self.ready_queue.push(task_id);
        self.state.total_tasks += 1;
        self.state.ready_tasks += 1;
    }

    pub fn get_task(&self, id: usize) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn get_task_mut(&mut self, id: usize) -> Option<&mut Task> {
        self.tasks.get_mut(id)
    }

    pub fn get_state(&self) -> &SchedulerState {
        &self.state
    }

    pub fn get_ready_queue(&self) -> &[usize] {
        &self.ready_queue
    }

    pub fn get_running_queue(&self) -> &[usize] {
        &self.running_queue
    }

    pub fn get_completed_queue(&self) -> &[usize] {
        &self.completed_queue
    }

    pub fn get_failed_queue(&self) -> &[usize] {
        &self.failed_queue
    }

    pub fn get_thread_pool(&self) -> &ThreadPool {
        &self.thread_pool
    }

    pub fn get_current_time(&self) -> u64 {
        self.current_time
    }
}
