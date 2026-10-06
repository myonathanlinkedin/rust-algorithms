use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

pub struct Task {
    // The future to be executed. Wrapped in a Mutex to allow mutable access across polls.
    pub future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,

}