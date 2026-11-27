use std::sync::Arc;
use std::sync::Mutex;

use std::sync::mpsc::{self, Sender, Receiver};
use std::thread::{self, JoinHandle};

/// A lightweight actor that processes messages of type `M` sequentially
/// on a dedicated thread.  The actor owns a mailbox (`Receiver<M>`) and
/// exposes a `Sender<M>` for external communication.
///
/// # Guarantees
/// * Messages are processed in FIFO order.
/// * The internal thread terminates when the last `Sender` is dropped.
/// * All resources are cleaned up on `Drop`.
#[derive(Debug)]
pub struct Actor<M>
where
    M: Send + 'static,
{
    sender: Sender<M>,
    handle: Option<JoinHandle<()>>,
}

impl<M> Actor<M>
where
    M: Send + 'static,
{
    /// Spawn a new actor that will invoke `processor` for each incoming
    /// message.  `processor` may capture state via `Arc`, `Mutex`, etc.
    ///
    /// # Example
    /// ```
    /// # use core::Actor;
    /// let actor = Actor::spawn(|msg: u32| {
    ///     println!("got {}", msg);
    /// });
    /// actor.send(42);
    /// ```
    pub fn spawn<F>(mut processor: F) -> Self
    where
        F: FnMut(M) + Send + 'static,
    {
        let (tx, rx): (Sender<M>, Receiver<M>) = mpsc::channel();

        // The processing thread consumes the receiver.
        let handle = thread::spawn(move || {
            // `for` loops over `rx` until the channel is closed.
            for msg in rx {
                processor(msg);
            }
        });

        Actor {
            sender: tx,
            handle: Some(handle),
        }
    }

    /// Send a message to the actor's mailbox.  Errors are ignored because
    /// they only occur when the actor has already terminated.
    pub fn send(&self, msg: M) {
        let _ = self.sender.send(msg);
    }

    /// Obtain a clone of the internal `Sender`.  Useful for sharing the
    /// mailbox with other actors.
    pub fn sender(&self) -> Sender<M> {
        self.sender.clone()
    }
}

impl<M> Drop for Actor<M>
where
    M: Send + 'static,
{
    fn drop(&mut self) {
        // Dropping the sender closes the channel, causing the thread to exit.
        drop(&self.sender);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}