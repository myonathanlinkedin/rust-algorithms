mod core;
use core::{SimpleExecutor, YieldNow};

use std::sync::{Arc, Mutex};

/// An async task that records its progress into a shared log.
async fn logged_task(id: usize, log: Arc<Mutex<Vec<String>>>) {
    {
        let mut guard = log.lock().unwrap();
        guard.push(format!("start{}", id));
    }
    YieldNow::new().await;
    {
        let mut guard = log.lock().unwrap();
        guard.push(format!("end{}", id));
    }
}

fn main() {
    // Demonstration similar to the unit test.
    let mut exec = SimpleExecutor::new();
    let log = Arc::new(Mutex::new(Vec::new()));
    exec.spawn(logged_task(0, Arc::clone(&log)));
    exec.spawn(logged_task(1, Arc::clone(&log)));
    exec.run();

    let result = log.lock().unwrap().clone();
    // Expected round‑robin order: start0, start1, end0, end1
    assert_eq!(result, vec!["start0", "start1", "end0", "end1"]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn executor_schedules_tasks_round_robin() {
        let mut exec = SimpleExecutor::new();
        let log = Arc::new(Mutex::new(Vec::new()));
        exec.spawn(logged_task(0, Arc::clone(&log)));
        exec.spawn(logged_task(1, Arc::clone(&log)));
        exec.spawn(logged_task(2, Arc::clone(&log)));
        exec.run();

        let result = log.lock().unwrap().clone();
        // Each task yields once, so we expect start0,start1,start2, then end0,end1,end2
        let expected = vec![
            "start0".to_string(),
            "start1".to_string(),
            "start2".to_string(),
            "end0".to_string(),
            "end1".to_string(),
            "end2".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn executor_handles_no_tasks_gracefully() {
        let mut exec = SimpleExecutor::new();
        exec.run(); // Should not panic
    }

    #[test]
    fn multiple_yields_per_task() {
        async fn double_yield(id: usize, log: Arc<Mutex<Vec<String>>>) {
            {
                let mut guard = log.lock().unwrap();
                guard.push(format!("begin{}", id));
            }
            YieldNow::new().await;
            YieldNow::new().await;
            {
                let mut guard = log.lock().unwrap();
                guard.push(format!("finish{}", id));
            }
        }

        let mut exec = SimpleExecutor::new();
        let log = Arc::new(Mutex::new(Vec::new()));
        exec.spawn(double_yield(0, Arc::clone(&log)));
        exec.spawn(double_yield(1, Arc::clone(&log)));
        exec.run();

        let result = log.lock().unwrap().clone();
        // Expected order: begin0, begin1, finish0, finish1
        let expected = vec![
            "begin0".to_string(),
            "begin1".to_string(),
            "finish0".to_string(),
            "finish1".to_string(),
        ];
        assert_eq!(result, expected);
    }
}