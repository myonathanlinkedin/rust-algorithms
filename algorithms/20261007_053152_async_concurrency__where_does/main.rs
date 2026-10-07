mod core;
use core::{Executor, YieldNow};

fn main() {
    // Simple demonstration: two tasks increment a shared counter.
    let executor = Executor::new();
    let counter = std::sync::Arc::new(std::sync::Mutex::new(0usize));

    let c1 = counter.clone();
    executor.spawn(async move {
        let mut lock = c1.lock().unwrap();
        *lock += 1;
    });

    let c2 = counter.clone();
    executor.spawn(async move {
        let mut lock = c2.lock().unwrap();
        *lock += 2;
    });

    executor.run();

    assert_eq!(*counter.lock().unwrap(), 3);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_basic_spawn_and_run() {
        let exec = Executor::new();
        let flag = Arc::new(Mutex::new(false));

        let f = flag.clone();
        exec.spawn(async move {
            let mut guard = f.lock().unwrap();
            *guard = true;
        });

        exec.run();
        assert!(*flag.lock().unwrap());
    }

    #[test]
    fn test_yield_now_interleaving() {
        let exec = Executor::new();
        let order = Arc::new(Mutex::new(Vec::new()));

        let o1 = order.clone();
        exec.spawn(async move {
            o1.lock().unwrap().push(1);
            YieldNow::default().await;
            o1.lock().unwrap().push(3);
        });

        let o2 = order.clone();
        exec.spawn(async move {
            o2.lock().unwrap().push(2);
        });

        exec.run();

        let result = order.lock().unwrap().clone();
        // Expected interleaving: 1,2,3 (because the first task yields after pushing 1)
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_multiple_yields() {
        let exec = Executor::new();
        let steps = Arc::new(Mutex::new(0usize));

        let s = steps.clone();
        exec.spawn(async move {
            for _ in 0..5 {
                YieldNow::default().await;
                let mut guard = s.lock().unwrap();
                *guard += 1;
            }
        });

        exec.run();
        assert_eq!(*steps.lock().unwrap(), 5);
    }

    #[test]
    fn test_nested_async_calls() {
        async fn inner() -> usize {
            42
        }

        async fn outer() -> usize {
            let v = inner().await;
            v + 1
        }

        let exec = Executor::new();
        let result = Arc::new(Mutex::new(0usize));
        let r = result.clone();

        exec.spawn(async move {
            let val = outer().await;
            *r.lock().unwrap() = val;
        });

        exec.run();
        assert_eq!(*result.lock().unwrap(), 43);
    }
}