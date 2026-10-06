mod core;
use core::{async_add, SimpleExecutor, YieldNow};

use std::sync::{Arc, Mutex};

fn main() {
    let mut exec = SimpleExecutor::new();
    let result = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result);
    exec.spawn(async move {
        let sum = async_add(2, 3).await;
        let mut lock = result_clone.lock().unwrap();
        *lock = Some(sum);
    });
    exec.run();
    let final_sum = result.lock().unwrap().expect("Result should be set");
    assert_eq!(final_sum, 5);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_async_add() {
        let mut exec = SimpleExecutor::new();
        let out = Arc::new(Mutex::new(0));
        let out_clone = Arc::clone(&out);
        exec.spawn(async move {
            let val = async_add(10, 15).await;
            let mut lock = out_clone.lock().unwrap();
            *lock = val;
        });
        exec.run();
        let got = *out.lock().unwrap();
        assert_eq!(got, 25);
    }

    #[test]
    fn test_yield_now() {
        let mut exec = SimpleExecutor::new();
        let flag = Arc::new(Mutex::new(false));
        let flag_clone = Arc::clone(&flag);
        exec.spawn(async move {
            YieldNow::new().await;
            let mut lock = flag_clone.lock().unwrap();
            *lock = true;
        });
        exec.run();
        assert!(*flag.lock().unwrap());
    }

    #[test]
    fn multiple_tasks() {
        let mut exec = SimpleExecutor::new();
        let results = Arc::new(Mutex::new(Vec::new()));
        for i in 0..5usize {
            let res_clone = Arc::clone(&results);
            exec.spawn(async move {
                let val = async_add(i as i32, (i * 2) as i32).await;
                let mut lock = res_clone.lock().unwrap();
                lock.push(val);
            });
        }
        exec.run();
        let mut got = results.lock().unwrap().clone();
        got.sort_unstable();
        assert_eq!(got, vec![0, 3, 6, 9, 12]);
    }
}