use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

mod core;
use core::{Executor, Task};

fn async_add(a: i32, b: i32) -> impl std::future::Future<Output = i32> + Send {
    async move { a + b }
}

fn async_yield_once() -> impl std::future::Future<Output = ()> + Send {
    struct YieldOnce {
        yielded: bool,
    }

    impl std::future::Future for YieldOnce {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<()> {
            if self.yielded {
                Poll::Ready(())
            } else {
                self.yielded = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    YieldOnce { yielded: false }
}

fn main() {
    // Simple demonstration that the scheduler (executor) runs tasks.
    let exec = Executor::new();

    let result = std::sync::Arc::new(std::sync::Mutex::new(None));
    let result_clone = result.clone();

    exec.spawn(async move {
        let sum = async_add(10, 32).await;
        *result_clone.lock().unwrap() = Some(sum);
    });

    exec.run();

    assert_eq!(*result.lock().unwrap(), Some(42));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_single_task() {
        let exec = Executor::new();
        let out = Arc::new(Mutex::new(0));
        let out_clone = out.clone();

        exec.spawn(async move {
            let val = async_add(7, 8).await;
            *out_clone.lock().unwrap() = val;
        });

        exec.run();
        assert_eq!(*out.lock().unwrap(), 15);
    }

    #[test]
    fn test_multiple_tasks() {
        let exec = Executor::new();
        let sums = Arc::new(Mutex::new(Vec::new()));
        let sums_clone1 = sums.clone();
        let sums_clone2 = sums.clone();

        exec.spawn(async move {
            let v = async_add(1, 2).await;
            sums_clone1.lock().unwrap().push(v);
        });

        exec.spawn(async move {
            let v = async_add(3, 4).await;
            sums_clone2.lock().unwrap().push(v);
        });

        exec.run();

        let mut collected = sums.lock().unwrap().clone();
        collected.sort_unstable();
        assert_eq!(collected, vec![3, 7]);
    }

    #[test]
    fn test_yielding_task() {
        let exec = Executor::new();
        let flag = Arc::new(Mutex::new(false));
        let flag_clone = flag.clone();

        exec.spawn(async move {
            async_yield_once().await;
            *flag_clone.lock().unwrap() = true;
        });

        // Before running, flag is false.
        assert!(!*flag.lock().unwrap());

        exec.run();

        // After executor finishes, flag should be true.
        assert!(*flag.lock().unwrap());
    }
}