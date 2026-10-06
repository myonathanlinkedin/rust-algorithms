use std::collections::VecDeque;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum TaskState {
    Ready,
    Running,
    Completed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: usize,
    pub state: TaskState,
    pub priority: u8,

}

pub struct Scheduler {
    pub ready_queue: VecDeque<Task>,
    pub running: Option<Task>,
    pub completed: Vec<Task>,
    pub total_scheduled: AtomicUsize,

}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            ready_queue: VecDeque::new(),
            running: None,
            completed: Vec::new(),
            total_scheduled: AtomicUsize::new(0),
        }
    }

    pub fn schedule(&mut self, task: Task) {
        self.total_scheduled.fetch_add(1, Ordering::SeqCst);
        self.ready_queue.push_back(task);
    }

    pub fn tick(&mut self) -> Option<&Task> {
        if self.running.is_some() {
            return self.running.as_ref();
        }
        if let Some(mut task) = self.ready_queue.pop_front() {
            task.state = TaskState::Running;
            self.running = Some(task);
            self.running.as_ref()
        } else {
            None
        }
    }

    pub fn complete_current(&mut self) -> Option<Task> {
        if let Some(mut task) = self.running.take() {
            task.state = TaskState::Completed;
            let completed_task = task.clone();
            self.completed.push(task);
            Some(completed_task)
        } else {
            None
        }
    }

    pub fn get_ready_queue_len(&self) -> usize {
        self.ready_queue.len()
    }

    pub fn get_completed(&self) -> &[Task] {
        &self.completed
    }

    pub fn get_total_scheduled(&self) -> usize {
        self.total_scheduled.load(Ordering::SeqCst)
    }

    pub fn is_idle(&self) -> bool {
        self.running.is_none() && self.ready_queue.is_empty()
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
