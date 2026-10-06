pub mod engine {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, RawWaker, RawWakerVTable, Waker};

    // Scheduler lives here. It drives futures to completion.
    #[derive(Debug, Default)]
    pub struct Scheduler;

    impl Scheduler {
        pub fn new() -> Self {
            Scheduler::default()
        }

        // Run a future to completion on the current thread.
        pub fn run<F>(&self, mut fut: F) -> F::Output
        where
            F: Future + Send,
        {
            // Safety: we provide a dummy waker that never wakes; the loop
            // continues until the future reports Ready.
            let waker = dummy_waker();
            let mut cx = Context::from_waker(&waker);
            // Pin the future on the stack.
            let mut fut = unsafe { Pin::new_unchecked(&mut fut) };
            loop {
                match fut.as_mut().poll(&mut cx) {
                    std::task::Poll::Ready(val) => return val,
                    std::task::Poll::Pending => {
                        // In this minimal scheduler we simply spin.
                        // Real schedulers would park the thread and be woken.
                        std::thread::yield_now();
                    }
                }
            }
        }
    }

    // Create a no-op waker suitable for a single-threaded executor.
    fn dummy_waker() -> Waker {
        // RawWakerVTable with no-op functions.
        const VTABLE: RawWakerVTable = RawWakerVTable::new(
            clone,
            wake,
            wake_by_ref,
            drop,
        );

        unsafe fn clone(data: *const ()) -> RawWaker {
            RawWaker::new(data, &VTABLE)
        }
        unsafe fn wake(_data: *const ()) {}
        unsafe fn wake_by_ref(_data: *const ()) {}
        unsafe fn drop(_data: *const ()) {}

        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }
}