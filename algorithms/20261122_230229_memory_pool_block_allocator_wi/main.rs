mod types;
mod engine;

use crate::types::{MemoryHandle, MemoryPool};

fn main() {
    // Basic sanity checks.
    let block_size = 64;
    let total_blocks = 8;
    let mut pool = MemoryPool::new(block_size, total_blocks);

    // Initially all blocks are free.
    assert_eq!(pool.free_count(), total_blocks);

    // Allocate all blocks.
    let mut handles: Vec<MemoryHandle> = Vec::new();
    for _ in 0..total_blocks {
        let h = pool.allocate().expect("Allocation should succeed");
        // Verify that the handle points inside the storage bounds.
        assert!(h.offset + h.size <= block_size * total_blocks);
        handles.push(h);
    }
    // Pool should be exhausted now.
    assert_eq!(pool.free_count(), 0);
    assert!(pool.allocate().is_none(), "Pool exhausted, allocation must return None");

    // Write distinct patterns into each allocated block.
    for (i, handle) in handles.iter().enumerate() {
        let slice = pool.get_mut(handle).expect("Mutable slice must be valid");
        for byte in slice.iter_mut() {
            *byte = i as u8;
        }
    }

    // Verify patterns.
    for (i, handle) in handles.iter().enumerate() {
        let slice = pool.get(handle).expect("Immutable slice must be valid");
        for &byte in slice.iter() {
            assert_eq!(byte, i as u8);
        }
    }

    // Deallocate even-indexed blocks.
    for (i, handle) in handles.iter().enumerate() {
        if i % 2 == 0 {
            let ok = pool.deallocate(handle.clone());
            assert!(ok, "Deallocation should succeed");
        }
    }
    // Half the blocks should be free now.
    assert_eq!(pool.free_count(), total_blocks / 2);

    // Allocate again; should reuse freed blocks.
    let mut new_handles = Vec::new();
    for _ in 0..(total_blocks / 2) {
        let h = pool.allocate().expect("Re-allocation should succeed");
        new_handles.push(h);
    }
    // No free blocks left again.
    assert_eq!(pool.free_count(), 0);

    // Ensure that newly allocated handles are distinct from previously kept handles.
    for nh in &new_handles {
        assert!(!handles.contains(nh), "New handle must not be in old handle list");
    }

    // Attempt double free detection.
    let duplicate = handles[0].clone();
    let result = pool.deallocate(duplicate);
    assert!(!result, "Double free must be rejected");

    // Clean up: deallocate all remaining blocks.
    for h in handles.iter().chain(new_handles.iter()) {
        // Some may already be freed; ignore failures.
        let _ = pool.deallocate(h.clone());
    }
    assert_eq!(pool.free_count(), total_blocks, "All blocks should be free after cleanup");
}

// End of file.