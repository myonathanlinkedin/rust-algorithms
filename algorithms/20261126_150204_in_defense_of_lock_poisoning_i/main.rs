use std::sync::Mutex;

mod core;
use core::PoisonableMutex;

fn main() {
    // Basic successful usage.
    let m = PoisonableMutex::new(0);
    {
        let mut guard = m.lock().unwrap();
        *guard = 5;
    }
    assert_eq!(*m.lock().unwrap(), 5);
    assert!(!m.is_poisoned());

    // Simulate a panic while holding the lock.
    let result = std::panic::catch_unwind(|| {
        let _guard = m.lock().unwrap();
        panic!("simulated panic while holding lock");
    });
    assert!(result.is_err(), "panic should have been caught");
    assert!(m.is_poisoned(), "mutex must be poisoned after panic");

    // Attempting to lock now yields an error.
    assert!(m.lock().is_err(), "locking a poisoned mutex should fail");

    // Recover by clearing the poison flag.
    m.clear_poison();
    assert!(!m.is_poisoned(), "poison flag cleared");

    // Lock works again.
    {
        let mut guard = m.lock().unwrap();
        *guard = 10;
    }
    assert_eq!(*m.lock().unwrap(), 10);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_normal_locking() {
        let m = PoisonableMutex::new(vec![1, 2, 3]);
        {
            let mut guard = m.lock().unwrap();
            guard.push(4);
        }
        let guard = m.lock().unwrap();
        assert_eq!(guard.as_slice(), &[1, 2, 3, 4]);
        assert!(!m.is_poisoned());
    }

    #[test]
    fn test_poison_detection() {
        let m = PoisonableMutex::new(42);
        let result = std::panic::catch_unwind(|| {
            let _guard = m.lock().unwrap();
            panic!("forced panic");
        });
        assert!(result.is_err());
        assert!(m.is_poisoned());
        assert!(matches!(m.lock(), Err(_)));
    }

    #[test]
    fn test_recovery() {
        let m = PoisonableMutex::new(String::from("ok"));
        let _ = std::panic::catch_unwind(|| {
            let _guard = m.lock().unwrap();
            panic!("another panic");
        });
        assert!(m.is_poisoned());

        m.clear_poison();
        assert!(!m.is_poisoned());

        {
            let mut guard = m.lock().unwrap();
            guard.push_str("ay");
        }
        assert_eq!(&*m.lock().unwrap(), "okay");
    }

    #[test]
    fn test_into_inner() {
        let m = PoisonableMutex::new(123);
        let inner = m.into_inner();
        assert_eq!(inner, 123);
    }
}