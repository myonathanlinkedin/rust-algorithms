use std::pin::Pin;
mod core;
use core::*;
use std::sync::{Arc, Mutex};

/// Example async task that yields once before recording its identifier.
async fn example_task(id: u32, out: Arc<Mutex<Vec<u32>>>) {
    YieldOnce::new().await;
    out.lock().unwrap().push(id);
}

fn main() {
    let mut exec = Executor::new();
    let results = Arc::new(Mutex::new(Vec::new()));

    exec.spawn(example_task(1, results.clone()));
    exec.spawn(example_task(2, results.clone()));
    exec.spawn(example_task(3, results.clone()));

    exec.run();

    let mut collected = results.lock().unwrap().clone();
    collected.sort_unstable();
    assert_eq!(collected, vec![1, 2, 3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yield_once() {
        let mut fut = Box::pin(YieldOnce::new());
        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        // First poll should be Pending.
        assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Pending));
        // Second poll should be Ready.
        assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Ready(())));
    }

    #[test]
    fn test_executor_runs_tasks() {
        let mut exec = Executor::new();
        let results = Arc::new(Mutex::new(Vec::new()));

        exec.spawn(example_task(10, results.clone()));
        exec.spawn(example_task(20, results.clone()));
        exec.spawn(example_task(30, results.clone()));

        exec.run();

        let mut out = results.lock().unwrap().clone();
        out.sort_unstable();
        assert_eq!(out, vec![10, 20, 30]);
    }

    #[test]
    fn test_multiple_yields() {
        // Future that yields twice before completing.
        #[derive(Debug, Clone, PartialEq)]
        struct YieldTwice {
            state: u8,
        }

        impl YieldTwice {
            fn new() -> Self {
                Self { state: 0 }
            }
        }

        impl Future for YieldTwice {
            type Output = u8;
            fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u8> {
                match self.state {
                    0 => {
                        self.state = 1;
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    1 => {
                        self.state = 2;
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                    _ => Poll::Ready(self.state),
                }
            }
        }

        async fn task_with_twice(yield_twice: YieldTwice, out: Arc<Mutex<Vec<u8>>>) {
            let v = yield_twice.await;
            out.lock().unwrap().push(v);
        }

        let mut exec = Executor::new();
        let results = Arc::new(Mutex::new(Vec::new()));
        exec.spawn(task_with_twice(YieldTwice::new(), results.clone()));
        exec.run();

        let out = results.lock().unwrap().clone();
        assert_eq!(out, vec![2]);
    }
}

// Helper to provide a no‑op waker for unit tests.
mod futures {
    use std::task::{RawWaker, RawWakerVTable, Waker};

    pub fn noop_waker() -> Waker {
        unsafe fn clone(_: *const ()) -> RawWaker {
            raw_waker()
        }
        unsafe fn wake(_: *const ()) {}
        unsafe fn wake_by_ref(_: *const ()) {}
        unsafe fn drop(_: *const ()) {}

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

        unsafe fn raw_waker() -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }

        unsafe { Waker::from_raw(raw_waker()) }
    }
}