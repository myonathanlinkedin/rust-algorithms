mod core;
use core::{Scheduler, Task, TaskState};

fn main() {
    test_scheduler_initial_state();
    test_schedule_and_tick();
    test_complete_current();
    test_multiple_tasks();
    test_idle_state();
    test_total_scheduled_counter();
    test_priority_ordering();
    test_empty_scheduler_tick();
    test_empty_scheduler_complete();
    test_task_state_transitions();
    test_scheduler_default();
    println!("All tests passed!");
}

fn test_scheduler_initial_state() {
    let scheduler = Scheduler::new();
    assert_eq!(scheduler.get_ready_queue_len(), 0);
    assert!(scheduler.is_idle());
    assert_eq!(scheduler.get_total_scheduled(), 0);
    assert!(scheduler.get_completed().is_empty());
}

fn test_schedule_and_tick() {
    let mut scheduler = Scheduler::new();
    let task = Task {
        id: 1,
        state: TaskState::Ready,
        priority: 5,
    };
    scheduler.schedule(task);
    assert_eq!(scheduler.get_ready_queue_len(), 1);
    assert!(!scheduler.is_idle());
    let running = scheduler.tick();
    assert!(running.is_some());
    let running_task = running.unwrap();
    assert_eq!(running_task.id, 1);
    assert_eq!(running_task.state, TaskState::Running);
}

fn test_complete_current() {
    let mut scheduler = Scheduler::new();
    let task = Task {
        id: 2,
        state: TaskState::Ready,
        priority: 3,
    };
    scheduler.schedule(task);
    scheduler.tick();
    let completed = scheduler.complete_current();
    assert!(completed.is_some());
    let completed_task = completed.unwrap();
    assert_eq!(completed_task.id, 2);
    assert_eq!(completed_task.state, TaskState::Completed);
    assert_eq!(scheduler.get_completed().len(), 1);
    assert!(scheduler.is_idle());
}

fn test_multiple_tasks() {
    let mut scheduler = Scheduler::new();
    for i in 0..5 {
        let task = Task {
            id: i,
            state: TaskState::Ready,
            priority: (i % 10) as u8,
        };
        scheduler.schedule(task);
    }
    assert_eq!(scheduler.get_ready_queue_len(), 5);
    for i in 0..5 {
        scheduler.tick();
        let completed = scheduler.complete_current();
        assert!(completed.is_some());
        assert_eq!(completed.unwrap().id, i);
    }
    assert_eq!(scheduler.get_completed().len(), 5);
    assert!(scheduler.is_idle());
}

fn test_idle_state() {
    let mut scheduler = Scheduler::new();
    assert!(scheduler.is_idle());
    let task = Task {
        id: 10,
        state: TaskState::Ready,
        priority: 1,
    };
    scheduler.schedule(task);
    assert!(!scheduler.is_idle());
    scheduler.tick();
    assert!(!scheduler.is_idle());
    scheduler.complete_current();
    assert!(scheduler.is_idle());
}

fn test_total_scheduled_counter() {
    let mut scheduler = Scheduler::new();
    assert_eq!(scheduler.get_total_scheduled(), 0);
    for i in 0..3 {
        let task = Task {
            id: i,
            state: TaskState::Ready,
            priority: 2,
        };
        scheduler.schedule(task);
    }
    assert_eq!(scheduler.get_total_scheduled(), 3);
}

fn test_priority_ordering() {
    let mut scheduler = Scheduler::new();
    let task_low = Task {
        id: 1,
        state: TaskState::Ready,
        priority: 1,
    };
    let task_high = Task {
        id: 2,
        state: TaskState::Ready,
        priority: 9,
    };
    scheduler.schedule(task_low);
    scheduler.schedule(task_high);
    let first = scheduler.tick().unwrap();
    assert_eq!(first.id, 1);
    scheduler.complete_current();
    let second = scheduler.tick().unwrap();
    assert_eq!(second.id, 2);
}

fn test_empty_scheduler_tick() {
    let mut scheduler = Scheduler::new();
    let result = scheduler.tick();
    assert!(result.is_none());
}

fn test_empty_scheduler_complete() {
    let mut scheduler = Scheduler::new();
    let result = scheduler.complete_current();
    assert!(result.is_none());
}

fn test_task_state_transitions() {
    let mut scheduler = Scheduler::new();
    let task = Task {
        id: 100,
        state: TaskState::Ready,
        priority: 7,
    };
    scheduler.schedule(task);
    let running = scheduler.tick().unwrap();
    assert_eq!(running.state, TaskState::Running);
    let completed = scheduler.complete_current().unwrap();
    assert_eq!(completed.state, TaskState::Completed);
}

fn test_scheduler_default() {
    let scheduler = Scheduler::default();
    assert!(scheduler.is_idle());
    assert_eq!(scheduler.get_total_scheduled(), 0);
}
