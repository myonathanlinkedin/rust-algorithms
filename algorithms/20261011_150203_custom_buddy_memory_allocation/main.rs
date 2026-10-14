mod core;
use core::BuddyAllocator;

fn main() {
    // Simple sanity checks executed at runtime
    let mut alloc = BuddyAllocator::new(1024, 32);
    let a = alloc.allocate(100).expect("alloc a");
    let b = alloc.allocate(200).expect("alloc b");
    assert!(alloc.deallocate(a));
    assert!(alloc.deallocate(b));
    assert_eq!(alloc.total_free_bytes(), 1024);
    assert!(alloc.check_consistency());
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_allocation_and_free() {
        let mut alloc = BuddyAllocator::new(512, 32);
        let p1 = alloc.allocate(50).expect("p1");
        let p2 = alloc.allocate(70).expect("p2");
        assert_ne!(p1, p2);
        assert!(alloc.deallocate(p1));
        assert!(alloc.deallocate(p2));
        assert_eq!(alloc.total_free_bytes(), 512);
        assert!(alloc.check_consistency());
    }

    #[test]
    fn allocate_exact_min_block() {
        let mut alloc = BuddyAllocator::new(256, 32);
        let p = alloc.allocate(1).expect("min");
        // usable region should be 1 byte, but block size is 32
        assert!(alloc.deallocate(p));
        assert_eq!(alloc.total_free_bytes(), 256);
    }

    #[test]
    fn allocate_full_memory() {
        let mut alloc = BuddyAllocator::new(128, 32);
        let p = alloc.allocate(120).expect("full");
        // block size will be 128 (order 2)
        assert!(alloc.deallocate(p));
        assert_eq!(alloc.total_free_bytes(), 128);
    }

    #[test]
    fn out_of_memory() {
        let mut alloc = BuddyAllocator::new(64, 16);
        let _ = alloc.allocate(30).expect("first");
        // second allocation cannot fit
        assert!(alloc.allocate(30).is_none());
    }

    #[test]
    fn coalescing_behavior() {
        let mut alloc = BuddyAllocator::new(256, 32);
        let a = alloc.allocate(30).unwrap(); // order 0 (32)
        let b = alloc.allocate(30).unwrap(); // order 0 (32)
        let c = alloc.allocate(30).unwrap(); // order 0 (32)
        let d = alloc.allocate(30).unwrap(); // order 0 (32)
        // Free a and b -> should coalesce into a 64‑byte block
        assert!(alloc.deallocate(a));
        assert!(alloc.deallocate(b));
        // Free c and d -> another 64‑byte block
        assert!(alloc.deallocate(c));
        assert!(alloc.deallocate(d));
        // Now free all 64‑byte blocks should coalesce into a 256‑byte block
        assert_eq!(alloc.total_free_bytes(), 256);
        assert!(alloc.check_consistency());
    }

    #[test]
    fn invalid_deallocation() {
        let mut alloc = BuddyAllocator::new(128, 32);
        // Random pointer not allocated
        assert!(!alloc.deallocate(10));
        // Double free
        let p = alloc.allocate(20).unwrap();
        assert!(alloc.deallocate(p));
        assert!(!alloc.deallocate(p));
    }
}