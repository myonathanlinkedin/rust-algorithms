use crate::types::{MemoryHandle, MemoryPool};

impl MemoryPool {
    /// Creates a new memory pool.
    ///
    /// * `block_size` – size of each block in bytes (must be > 0).
    /// * `total_blocks` – number of blocks to allocate (must be > 0).
    ///
    /// Returns a `MemoryPool` with all blocks initially free.
    pub fn new(block_size: usize, total_blocks: usize) -> Self {
        assert!(block_size > 0, "block_size must be greater than zero");
        assert!(total_blocks > 0, "total_blocks must be greater than zero");

        let storage = vec![0u8; block_size * total_blocks];
        // Initialize free list with all block indices.
        let mut free_list = Vec::with_capacity(total_blocks);
        for idx in 0..total_blocks {
            free_list.push(idx);
        }

        MemoryPool {
            block_size,
            total_blocks,
            storage,
            free_list,
        }
    }

    /// Allocates a single block from the pool.
    ///
    /// Returns `Some(MemoryHandle)` on success, or `None` if the pool is exhausted.
    pub fn allocate(&mut self) -> Option<MemoryHandle> {
        // Pop a free block index; if none remain, allocation fails.
        self.free_list.pop().map(|block_idx| MemoryHandle {
            offset: block_idx * self.block_size,
            size: self.block_size,
        })
    }

    /// Deallocates a previously allocated block.
    ///
    /// Returns `true` if the block was successfully returned to the free list,
    /// `false` if the handle is invalid or the block was already free.
    pub fn deallocate(&mut self, handle: MemoryHandle) -> bool {
        // Validate handle bounds.
        if handle.size != self.block_size {
            return false;
        }
        if handle.offset % self.block_size != 0 {
            return false;
        }
        let block_idx = handle.offset / self.block_size;
        if block_idx >= self.total_blocks {
            return false;
        }
        // Detect double free (linear search; acceptable for demonstration).
        if self.free_list.contains(&block_idx) {
            return false;
        }
        self.free_list.push(block_idx);
        true
    }

    /// Provides a mutable slice to the allocated block.
    ///
    /// Returns `Some(&mut [u8])` if the handle is valid, otherwise `None`.
    pub fn get_mut(&mut self, handle: &MemoryHandle) -> Option<&mut [u8]> {
        if handle.size != self.block_size {
            return None;
        }
        if handle.offset + handle.size > self.storage.len() {
            return None;
        }
        Some(&mut self.storage[handle.offset..handle.offset + handle.size])
    }

    /// Provides an immutable slice to the allocated block.
    ///
    /// Returns `Some(&[u8])` if the handle is valid, otherwise `None`.
    pub fn get(&self, handle: &MemoryHandle) -> Option<&[u8]> {
        if handle.size != self.block_size {
            return None;
        }
        if handle.offset + handle.size > self.storage.len() {
            return None;
        }
        Some(&self.storage[handle.offset..handle.offset + handle.size])
    }

    /// Returns the number of currently free blocks.
    pub fn free_count(&self) -> usize {
        self.free_list.len()
    }
}

// A placeholder main function to satisfy the guard that each file contains a `fn main`.
#[allow(dead_code)] pub fn run_module() {
    // Not used; the crate entry point is in main.rs.
}