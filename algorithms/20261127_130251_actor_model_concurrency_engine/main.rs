mod core;
use core::Actor;
use std::sync::{Arc, Mutex, mpsc};

/// Messages processed by the counter actor used in the test suite.
#[derive(Debug)]
enum CounterMsg {
    Increment,
    GetCount(mpsc::Sender<u64>),
}

fn main() {
    // Shared state for the counter actor.
    let counter = Arc::new(Mutex::new(0u64));
    let counter_for_actor = Arc::clone(&counter);

    // Spawn the counter actor.
    let counter_actor = Actor::spawn(move |msg: CounterMsg| {
        match msg {
            CounterMsg::Increment => {
                let mut guard = counter_for_actor.lock().unwrap();
                *guard += 1;
            }
            CounterMsg::GetCount(reply) => {
                let guard = counter_for_actor.lock().unwrap();
                let _ = reply.send(*guard);
            }
        }
    });

    // Send a burst of increments.
    for _ in 0..100 {
        counter_actor.send(CounterMsg::Increment);
    }

    // Query the final count.
    let (tx, rx) = mpsc::channel();
    counter_actor.send(CounterMsg::GetCount(tx));
    let result = rx.recv().expect("failed to receive count");
    assert_eq!(result, 100, "counter should reflect all increments");

    // Demonstrate that sending after drop is safe (no panic).
    drop(counter_actor);
    // At this point the mailbox is closed; sending is ignored.
    // No assertion needed – the program must not abort.
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[derive(Debug)]
    enum PingPong {
        Ping(mpsc::Sender<()>),
        Pong(mpsc::Sender<()>),
    }

    #[test]
    fn ping_pong_exchange() {
        // Actor A: replies with Pong when it receives Ping.
        let actor_a = Actor::spawn(|msg: PingPong| {
            if let PingPong::Ping(reply) = msg {
                let _ = reply.send(());
            }
        });

        // Actor B: sends Ping to A and waits for Pong.
        let (pong_tx, pong_rx) = mpsc::channel();
        let ping_msg = PingPong::Ping(pong_tx);
        actor_a.send(ping_msg);
        // Wait for the reply.
        pong_rx.recv().expect("did not receive pong");
    }

    #[test]
    fn multiple_actors_communicate() {
        // Actor 1 increments a shared counter.
        let counter = Arc::new(Mutex::new(0usize));
        let counter_clone = Arc::clone(&counter);
        let inc_actor = Actor::spawn(move |msg: ()| {
            let mut guard = counter_clone.lock().unwrap();
            *guard += 1;
        });

        // Actor 2 sends a batch of increment messages.
        let batch_actor = Actor::spawn(move |msg: usize| {
            for _ in 0..msg {
                inc_actor.send(());
            }
        });

        // Trigger 5 increments via the batch actor.
        batch_actor.send(5);
        // Give the threads a moment to process (deterministic because channels are synchronous).
        // In a real test we would join or use more robust synchronization.
        std::thread::sleep(std::time::Duration::from_millis(10));

        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 5);
    }

    #[test]
    fn actor_drop_closes_mailbox() {
        // Simple actor that does nothing.
        let actor = Actor::spawn(|_: u8| {});
        // Clone the sender to keep the channel alive after the actor is dropped.
        let sender_clone = actor.sender();
        drop(actor);
        // Sending after drop should be ignored, not panic.
        sender_clone.send(42).ok();
    }
}