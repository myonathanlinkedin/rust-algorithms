mod core;
use core::{async_increment, Executor};
use std::sync::{Arc, Mutex};

fn main() {
    // Shared state to be mutated by concurrent async tasks.
    let counter = Arc::new(Mutex::new(0usize));

    // Build the executor and schedule two independent tasks.
    let mut exec = Executor::new();
    exec.spawn(async_increment(counter.clone(), 5));
    exec.spawn(async_increment(counter.clone(), 5));

    // Run the scheduler until all tasks finish.
    exec.run();

    // Verify that the scheduler correctly interleaved the tasks.
    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 10, "Counter should be incremented 10 times");

    // Additional sanity check: running an empty executor does nothing.
    let mut empty_exec = Executor::new();
    empty_exec.run(); // should not panic

    println!("All assertions passed.");
}