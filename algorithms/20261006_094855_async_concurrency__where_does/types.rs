use std::future::Future;
use std::pin::Pin;

/// A unit of asynchronous work.
///
/// The `future` field holds the async computation that the executor will poll.
pub struct Task {
    pub id: usize,
    pub future: Pin<Box<dyn Future<Output = ()> + Send>>,

}
