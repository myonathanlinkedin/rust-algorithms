pub struct MemoryPool {
    pub block_size: usize,
    pub capacity: usize,
    pub storage: Vec<u8>,
    pub free_list: Vec<usize>, // stores block indices

}

impl MemoryPool {
    /// Create a new memory pool.
    ///
    /// * `block_size` – size of each block in bytes, must be > 0.
    /// * `capacity` – number of blocks the pool can hold, must be > 0.
    ///
    /// Panics if either argument is zero.
    pub fn new(block_size: usize, capacity: usize) -> Self {
        assert!(block_size > 0, "block_size must be non‑zero");
        assert!(capacity > 0, "capacity must be non‑zero");
        let storage = vec![0u8; block_size * capacity];
        // Initialise free list with all block indices (0..capacity-1)
        let mut free_list = Vec::with_capacity(capacity);
        for idx in (0..capacity).rev() {
            free_list.push(idx);
        }
        MemoryPool {
            block_size,
            capacity,
            storage,
            free_list,
        }
    }

    /// Return the size of each block.
    #[inline]
    pub fn block_size(&self) -> usize {
        self.block_size
    }

    /// Return the total number of blocks the pool can manage.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Return the number of blocks currently available for allocation.
    #[inline]
    pub fn available(&self) -> usize {
        self.free_list.len()
    }

    /// Allocate a block and obtain its handle.
    ///
    /// Returns `Some(handle)` on success, where `handle` is the block index.
    /// Returns `None` if the pool is exhausted.
    pub fn allocate(&mut self) -> Option<usize> {
        self.free_list.pop()
    }

    /// Deallocate a previously allocated block.
    ///
    /// * `handle` – the block index returned by `allocate`.
    ///
    /// Returns `Ok(())` on success, or an error string if the handle is
    /// out of range or already free.
    pub fn deallocate(&mut self, handle: usize) -> Result<(), &'static str> {
        if handle >= self.capacity {
            return Err("handle out of range");
        }
        if self.free_list.contains(&handle) {
            return Err("double free detected");
        }
        self.free_list.push(handle);
        Ok(())
    }

    /// Obtain a mutable slice to the block identified by `handle`.
    ///
    /// Returns `Some(&mut [u8])` if the handle is valid and currently allocated,
    /// otherwise `None`.
    pub fn get_mut(&mut self, handle: usize) -> Option<&mut [u8]> {
        if handle >= self.capacity {
            return None;
        }
        // If the block is in the free list, it is not allocated.
        if self.free_list.contains(&handle) {
            return None;
        }
        let start = handle * self.block_size;
        let end = start + self.block_size;
        self.storage.get_mut(start..end)
    }

    /// Obtain an immutable slice to the block identified by `handle`.
    ///
    /// Returns `Some(&[u8])` if the handle is valid and currently allocated,
    /// otherwise `None`.
    pub fn get(&self, handle: usize) -> Option<&[u8]> {
        if handle >= self.capacity {
            return None;
        }
        if self.free_list.contains(&handle) {
            return None;
        }
        let start = handle * self.block_size;
        let end = start + self.block_size;
        self.storage.get(start..end)
    }
}