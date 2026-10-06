mod core;
use core::{compute_sum, Executor, YieldOnce};

fn main() {
    // Simple demonstration that the executor runs the example future.
    let mut exec = Executor::new();
    let result = std::sync::Arc::new(std::sync::Mutex::new(None));
    let result_clone = result.clone();

    exec.spawn(async move {
        let val = compute_sum().await;
        *result_clone.lock().unwrap() = Some(val);
    });

    exec.run();

    let final_val = result.lock().unwrap().expect("future should have set a value");
    assert_eq!(final_val, 3);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_yield_once() {
        let mut exec = Executor::new();
        let mut counter = 0;
        let counter_arc = Arc::new(Mutex::new(&mut counter));
        let counter_clone = counter_arc.clone();

        exec.spawn(async move {
            // The future yields twice before completing.
            YieldOnce::new().await;
            YieldOnce::new().await;
            let mut guard = counter_clone.lock().unwrap();
            **guard += 1;
        });

        exec.run();

        let guard = counter_arc.lock().unwrap();
        assert_eq!(**guard, 1);
    }

    #[test]
    fn test_compute_sum() {
        let mut exec = Executor::new();
        let result = Arc::new(Mutex::new(None));
        let result_clone = result.clone();

        exec.spawn(async move {
            let val = compute_sum().await;
            *result_clone.lock().unwrap() = Some(val);
        });

        exec.run();

        let got = result.lock().unwrap().expect("result should be set");
        assert_eq!(got, 3); // 0 + 1 + 2 = 3
    }

    #[test]
    fn test_multiple_tasks() {
        let mut exec = Executor::new();
        let res1 = Arc::new(Mutex::new(None));
        let res2 = Arc::new(Mutex::new(None));

        let r1 = res1.clone();
        exec.spawn(async move {
            let v = compute_sum().await;
            *r1.lock().unwrap() = Some(v);
        });

        let r2 = res2.clone();
        exec.spawn(async move {
            let mut sum = 0;
            for _ in 0..2 {
                YieldOnce::new().await;
                sum += 5;
            }
            *r2.lock().unwrap() = Some(sum);
        });

        exec.run();

        assert_eq!(res1.lock().unwrap().unwrap(), 3);
        assert_eq!(res2.lock().unwrap().unwrap(), 10);
    }
}