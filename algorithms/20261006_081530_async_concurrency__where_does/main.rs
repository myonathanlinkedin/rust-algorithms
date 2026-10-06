mod core;
use core::{Scheduler, Task, TaskState};

fn main() {
    // Test 1: Basic task spawning and execution
    let mut scheduler = Scheduler::new();
    let task1 = Task::new(1, 5);
    let task2 = Task::new(2, 3);
    scheduler.spawn(task1);
    scheduler.spawn(task2);
    assert_eq!(scheduler.ready_count(), 2);
    assert!(!scheduler.is_idle());

    // Test 2: Priority-based scheduling
    let executed = scheduler.step();
    assert!(executed.is_some());
    let executed_task = executed.unwrap();
    assert_eq!(executed_task.id, 1); // Higher priority task should run first
    assert_eq!(executed_task.state, TaskState::Running);
    assert_eq!(executed_task.execution_time, 1);

    // Test 3: Completing a task
    let completed = scheduler.complete_current();
    assert!(completed.is_some());
    let completed_task = completed.unwrap();
    assert_eq!(completed_task.state, TaskState::Completed);
    assert_eq!(scheduler.completed_count(), 1);
    assert_eq!(scheduler.ready_count(), 1);

    // Test 4: Suspend and resume
    let _ = scheduler.step(); // Run the second task
    let suspended = scheduler.suspend_current();
    assert!(suspended.is_some());
    let suspended_task = suspended.unwrap();
    assert_eq!(suspended_task.state, TaskState::Suspended);
    assert_eq!(scheduler.suspended_count(), 1);

    let resumed = scheduler.resume(2);
    assert!(resumed.is_some());
    let resumed_task = resumed.unwrap();
    assert_eq!(resumed_task.state, TaskState::Ready);
    assert_eq!(scheduler.ready_count(), 1);
    assert_eq!(scheduler.suspended_count(), 0);

    // Test 5: Run until complete
    let mut scheduler2 = Scheduler::new();
    for i in 0..5 {
        scheduler2.spawn(Task::new(i, (i % 3) as u8));
    }
    let results = scheduler2.run_until_complete();
    assert_eq!(results.len(), 5);
    assert_eq!(scheduler2.completed_count(), 5);
    assert!(scheduler2.is_idle());

    // Test 6: Empty scheduler behavior
    let mut empty_scheduler = Scheduler::new();
    assert!(empty_scheduler.is_idle());
    assert_eq!(empty_scheduler.step(), None);
    assert_eq!(empty_scheduler.complete_current(), None);
    assert_eq!(empty_scheduler.suspend_current(), None);
    assert_eq!(empty_scheduler.resume(999), None);

    // Test 7: Total executed counter
    let mut scheduler3 = Scheduler::new();
    scheduler3.spawn(Task::new(1, 1));
    scheduler3.spawn(Task::new(2, 2));
    scheduler3.step();
    scheduler3.complete_current();
    scheduler3.step();
    scheduler3.complete_current();
    assert_eq!(scheduler3.total_executed(), 2);

    // Test 8: Multiple suspends and resumes
    let mut scheduler4 = Scheduler::new();
    for i in 0..3 {
        scheduler4.spawn(Task::new(i, 1));
    }
    scheduler4.step();
    scheduler4.suspend_current();
    scheduler4.step();
    scheduler4.suspend_current();
    assert_eq!(scheduler4.suspended_count(), 2);
    scheduler4.resume(0);
    scheduler4.resume(1);
    assert_eq!(scheduler4.ready_count(), 2);
    assert_eq!(scheduler4.suspended_count(), 0);

    println!("All tests passed successfully!");
}
