use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct TokenBucket {
    pub capacity: f64,
    pub tokens: f64,
    pub refill_rate: f64, // tokens per second
    pub last_refill: Instant,

}

impl TokenBucket {
    pub fn new(capacity: u32, refill_rate: f64) -> Self {
        let now = Instant::now();
        TokenBucket {
            capacity: capacity as f64,
            tokens: capacity as f64,
            refill_rate,
            last_refill: now,
        }
    }

    fn refill(&mut self, now: Instant) {
        let elapsed = now.duration_since(self.last_refill);
        let added = elapsed.as_secs_f64() * self.refill_rate;
        if added > 0.0 {
            self.tokens = (self.tokens + added).min(self.capacity);
            self.last_refill = now;
        }
    }

    pub fn try_acquire(&mut self, now: Instant) -> bool {
        self.refill(now);
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    pub fn available(&self) -> f64 {
        self.tokens
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlidingWindowLimiter {
    pub max_requests: usize,
    pub window: Duration,
    pub timestamps: VecDeque<Instant>,

}

impl SlidingWindowLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        SlidingWindowLimiter {
            max_requests,
            window,
            timestamps: VecDeque::new(),
        }
    }

    pub fn try_acquire(&mut self, now: Instant) -> bool {
        while let Some(&front) = self.timestamps.front() {
            if now.duration_since(front) > self.window {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        if self.timestamps.len() < self.max_requests {
            self.timestamps.push_back(now);
            true
        } else {
            false
        }
    }

    pub fn current_count(&self) -> usize {
        self.timestamps.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DistributedRateLimiter {
    pub buckets: Vec<TokenBucket>,

}

impl DistributedRateLimiter {
    pub fn new(buckets: Vec<TokenBucket>) -> Self {
        DistributedRateLimiter { buckets }
    }

    pub fn try_acquire(&mut self, now: Instant) -> bool {
        for bucket in &mut self.buckets {
            if bucket.try_acquire(now) {
                return true;
            }
        }
        false
    }

    pub fn total_available(&self) -> f64 {
        self.buckets.iter().map(|b| b.available()).sum()
    }
}