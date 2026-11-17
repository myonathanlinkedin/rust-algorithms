mod core;
use core::MemoryPool;

fn main() {
    // Simple sanity checks executed at runtime.
    let mut pool = MemoryPool::new(16, 4);
    assert_eq!(pool.block_size(), 16);
    assert_eq!(pool.capacity(), 4);
    assert_eq!(pool.available(), 4);

    // Allocate all blocks.
    let mut handles = Vec::new();
    for _ in 0..4 {
        let h = pool.allocate().expect("should allocate");
        handles.push(h);
    }
    assert_eq!(pool.available(), 0);

    // Attempting a fifth allocation must fail.
    assert!(pool.allocate().is_none());

    // Write distinct patterns into each block.
    for (i, &h) in handles.iter().enumerate() {
        let slice = pool.get_mut(h).expect("allocated block must be mutable");
        for byte in slice.iter_mut() {
            *byte = i as u8;
        }
    }

    // Verify patterns.
    for (i, &h) in handles.iter().enumerate() {
        let slice = pool.get(h).expect("allocated block must be readable");
        for &byte in slice {
            assert_eq!(byte, i as u8);
        }
    }

    // Deallocate two blocks and reallocate them.
    let reclaimed = handles.drain(0..2).collect::<Vec<_>>();
    for h in reclaimed.iter() {
        pool.deallocate(*h).expect("valid deallocation");
    }
    assert_eq!(pool.available(), 2);

    // New allocations should reuse freed indices (order not guaranteed).
    let new1 = pool.allocate().expect("should allocate after free");
    let new2 = pool.allocate().expect("should allocate after free");
    assert_eq!(pool.available(), 0);
    // Ensure the new handles are among the reclaimed ones.
    assert!(reclaimed.contains(&new1));
    assert!(reclaimed.contains(&new2));

    // Clean up.
    for h in handles.iter().chain([new1, new2].iter()) {
        pool.deallocate(*h).expect("final deallocation");
    }
    assert_eq!(pool.available(), 4);
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_and_properties() {
        let pool = MemoryPool::new(32, 10);
        assert_eq!(pool.block_size(), 32);
        assert_eq!(pool.capacity(), 10);
        assert_eq!(pool.available(), 10);
    }

    #[test]
    fn test_allocate_deallocate_cycle() {
        let mut pool = MemoryPool::new(8, 2);
        let h1 = pool.allocate().expect("first allocation");
        let h2 = pool.allocate().expect("second allocation");
        assert!(pool.allocate().is_none()); // pool exhausted

        // Deallocate first block and allocate again.
        pool.deallocate(h1).expect("deallocate h1");
        assert_eq!(pool.available(), 1);
        let h3 = pool.allocate().expect("re‑allocation");
        assert_eq!(h3, h1); // should reuse the freed index
        assert!(pool.allocate().is_none());

        // Clean up.
        pool.deallocate(h2).unwrap();
        pool.deallocate(h3).unwrap();
        assert_eq!(pool.available(), 2);
    }

    #[test]
    fn test_invalid_deallocate() {
        let mut pool = MemoryPool::new(4, 1);
        let handle = pool.allocate().unwrap();
        // Double free.
        pool.deallocate(handle).unwrap();
        assert_eq!(pool.deallocate(handle), Err("double free detected"));
        // Out‑of‑range handle.
        assert_eq!(pool.deallocate(999), Err("handle out of range"));
    }

    #[test]
    fn test_mutation_and_isolation() {
        let mut pool = MemoryPool::new(5, 3);
        let h0 = pool.allocate().unwrap();
        let h1 = pool.allocate().unwrap();

        {
            let slice0 = pool.get_mut(h0).unwrap();
            slice0.copy_from_slice(&[1, 2, 3, 4, 5]);
        }
        {
            let slice1 = pool.get_mut(h1).unwrap();
            slice1.copy_from_slice(&[5, 4, 3, 2, 1]);
        }

        let read0 = pool.get(h0).unwrap();
        let read1 = pool.get(h1).unwrap();
        assert_eq!(read0, &[1, 2, 3, 4, 5]);
        assert_eq!(read1, &[5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_get_on_free_block_returns_none() {
        let mut pool = MemoryPool::new(10, 2);
        let h = pool.allocate().unwrap();
        assert!(pool.get(h).is_some());
        pool.deallocate(h).unwrap();
        assert!(pool.get(h).is_none());
        assert!(pool.get_mut(h).is_none());
    }

    #[test]
    fn test_full_exhaustion_and_reuse() {
        let mut pool = MemoryPool::new(1, 3);
        let handles: Vec<_> = (0..3).filter_map(|_| pool.allocate()).collect();
        assert_eq!(handles.len(), 3);
        assert_eq!(pool.available(), 0);
        for h in &handles {
            pool.deallocate(*h).unwrap();
        }
        assert_eq!(pool.available(), 3);
        // Allocate again; all handles should be valid.
        for _ in 0..3 {
            assert!(pool.allocate().is_some());
        }
    }
}