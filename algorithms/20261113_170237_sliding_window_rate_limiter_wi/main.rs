use std::time::{Duration, Instant};

mod core;
use core::{TokenBucket, SlidingWindowLimiter, DistributedRateLimiter};

fn main() {
    // Basic sanity checks
    let mut tb = TokenBucket::new(5, 1.0);
    let now = Instant::now();
    assert!(tb.try_acquire(now));
    assert_eq!(tb.available(), 4.0);

    let mut sw = SlidingWindowLimiter::new(2, Duration::from_secs(1));
    let t0 = Instant::now();
    assert!(sw.try_acquire(t0));
    assert!(sw.try_acquire(t0 + Duration::from_millis(100)));
    assert!(!sw.try_acquire(t0 + Duration::from_millis(200)));

    let mut dr = DistributedRateLimiter::new(vec![
        TokenBucket::new(1, 0.0),
        TokenBucket::new(2, 0.0),
    ]);
    let t = Instant::now();
    assert!(dr.try_acquire(t));
    assert!(dr.try_acquire(t));
    assert!(dr.try_acquire(t));
    assert!(!dr.try_acquire(t));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn token_bucket_basic() {
        let mut bucket = TokenBucket::new(3, 2.0);
        let start = Instant::now();
        assert!(bucket.try_acquire(start));
        assert!(bucket.try_acquire(start));
        assert!(bucket.try_acquire(start));
        assert!(!bucket.try_acquire(start));

        let later = start + Duration::from_millis(500);
        assert!(bucket.try_acquire(later));
        assert!(!bucket.try_acquire(later));

        let later2 = later + Duration::from_secs(1);
        assert!(bucket.try_acquire(later2));
        assert!(bucket.try_acquire(later2));
        assert!(!bucket.try_acquire(later2));
    }

    #[test]
    fn sliding_window_basic() {
        let mut limiter = SlidingWindowLimiter::new(3, Duration::from_secs(2));
        let base = Instant::now();
        assert!(limiter.try_acquire(base));
        assert!(limiter.try_acquire(base + Duration::from_millis(500)));
        assert!(limiter.try_acquire(base + Duration::from_millis(1000)));
        assert!(!limiter.try_acquire(base + Duration::from_millis(1500)));

        let later = base + Duration::from_millis(2100);
        assert!(limiter.try_acquire(later));
    }

    #[test]
    fn distributed_limiter_basic() {
        let mut dr = DistributedRateLimiter::new(vec![
            TokenBucket::new(1, 0.0),
            TokenBucket::new(2, 0.0),
        ]);
        let now = Instant::now();
        assert_eq!(dr.total_available(), 3.0);
        assert!(dr.try_acquire(now));
        assert_eq!(dr.total_available(), 2.0);
        assert!(dr.try_acquire(now));
        assert!(dr.try_acquire(now));
        assert_eq!(dr.total_available(), 0.0);
        assert!(!dr.try_acquire(now));
    }

    #[test]
    fn distributed_refill_behavior() {
        let mut dr = DistributedRateLimiter::new(vec![
            TokenBucket::new(2, 1.0),
            TokenBucket::new(2, 1.0),
        ]);
        let start = Instant::now();
        for _ in 0..4 {
            assert!(dr.try_acquire(start));
        }
        assert!(!dr.try_acquire(start));

        let later = start + Duration::from_millis(1500);
        assert!(dr.try_acquire(later));
        assert!(dr.try_acquire(later));
        assert!(dr.try_acquire(later));
        assert!(!dr.try_acquire(later));
    }
}