use std::pin::Pin;
mod core;
use core::{async_yield, Executor};

use std::sync::{Arc, Mutex};

/// Increment a shared counter `n` times, yielding after each increment.
async fn increment(counter: Arc<Mutex<u32>>, n: u32) {
    for _ in 0..n {
        {
            let mut guard = counter.lock().unwrap();
            *guard += 1;
        }
        async_yield().await;
    }
}

fn main() {
    let exec = Executor::new();
    let counter = Arc::new(Mutex::new(0u32));

    exec.spawn(increment(counter.clone(), 5));
    exec.spawn(increment(counter.clone(), 7));

    exec.run();

    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 12);
    println!("All tasks completed, counter = {}", final_count);
}

// ---------- Unit tests ----------
#[cfg(test)]
mod tests {
    use super::*;
    use core::YieldNow;
    use std::future::Future;
    use std::task::{Context, Poll};

    #[test]
    fn test_yield_now() {
        let mut fut = YieldNow::new();
        let waker = futures::task::noop_waker(); // using std's noop waker via futures crate is not allowed
        // Instead, create a dummy waker using std's raw waker utilities.
        fn dummy_waker() -> std::task::Waker {
            use std::task::{RawWaker, RawWakerVTable, Waker};
            unsafe fn clone(_: *const ()) -> RawWaker {
                raw_waker()
            }
            unsafe fn wake(_: *const ()) {}
            unsafe fn wake_by_ref(_: *const ()) {}
            unsafe fn drop(_: *const ()) {}
            unsafe fn raw_waker() -> RawWaker {
                RawWaker::new(std::ptr::null(), &VTABLE)
            }
            static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
            unsafe { Waker::from_raw(raw_waker()) }
        }
        let waker = dummy_waker();
        let mut cx = Context::from_waker(&waker);
        // First poll should be Pending.
        match Pin::new(&mut fut).poll(&mut cx) {
            Poll::Pending => {}
            _ => panic!("First poll should be Pending"),
        }
        // Second poll should be Ready.
        match Pin::new(&mut fut).poll(&mut cx) {
            Poll::Ready(()) => {}
            _ => panic!("Second poll should be Ready"),
        }
    }

    #[test]
    fn test_executor_multiple_tasks() {
        let exec = Executor::new();
        let counter = Arc::new(Mutex::new(0u32));

        exec.spawn(increment(counter.clone(), 3));
        exec.spawn(increment(counter.clone(), 4));
        exec.spawn(increment(counter.clone(), 5));

        exec.run();

        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 12);
    }

    #[test]
    fn test_async_yield_behavior() {
        async fn simple() -> u32 {
            let mut x = 0;
            async_yield().await;
            x += 1;
            async_yield().await;
            x + 1
        }

        let exec = Executor::new();
        let result = Arc::new(Mutex::new(None));
        let result_clone = result.clone();

        exec.spawn(async move {
            let val = simple().await;
            *result_clone.lock().unwrap() = Some(val);
        });

        exec.run();

        let val = result.lock().unwrap().take().unwrap();
        assert_eq!(val, 2);
    }
}

// === END OF main.rs ===