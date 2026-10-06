mod types;
mod engine;

use types::types::{async_add, async_sequence, ComputationResult};
use engine::engine::Scheduler;

fn main() {
    // Instantiate the scheduler (the "where the scheduler lives").
    let scheduler = Scheduler::new();

    // Test 1: simple addition.
    let result: ComputationResult = scheduler.run(async_add(2, 3));
    assert_eq!(result, ComputationResult { sum: 5 });
    println!("Test 1 passed: {:?}", result);

    // Test 2: sequence sum.
    let values = [10, 20, 30];
    let result_seq: ComputationResult = scheduler.run(async_sequence(&values));
    assert_eq!(result_seq, ComputationResult { sum: 60 });
    println!("Test 2 passed: {:?}", result_seq);

    // Test 3: multiple sequential runs to ensure scheduler can be reused.
    for i in 0..5 {
        let res = scheduler.run(async_add(i, i * 2));
        assert_eq!(res, ComputationResult { sum: i + i * 2 });
    }
    println!("Test 3 passed: multiple sequential runs succeeded.");

    // Edge case: empty slice for async_sequence.
    let empty: [i32; 0] = [];
    let empty_res = scheduler.run(async_sequence(&empty));
    assert_eq!(empty_res, ComputationResult { sum: 0 });
    println!("Edge case test passed: empty slice handled correctly.");
}