use std::cmp::max;

#[derive(Debug, Clone, PartialEq)]
pub struct BuddyAllocator {
    /// Total size of the memory pool (must be power of two)
    pub total_size: usize,
    /// Minimum allocatable block size (must be power of two)
    pub min_block: usize,
    /// Maximum order (log2(total_size / min_block))
    pub max_order: usize,
    /// Simulated memory pool
    pub memory: Vec<u8>,
    /// Free lists indexed by order; order 0 => block size = min_block
    pub free_lists: Vec<Vec<usize>>,

}

impl BuddyAllocator {
    /// Create a new buddy allocator.
    /// `total_size` and `min_block` must be powers of two and `total_size >= min_block`.
    pub fn new(total_size: usize, min_block: usize) -> Self {
        assert!(total_size.is_power_of_two(), "total_size must be power of two");
        assert!(min_block.is_power_of_two(), "min_block must be power of two");
        assert!(total_size >= min_block, "total_size must be >= min_block");

        let max_order = (total_size / min_block).trailing_zeros() as usize;
        let mut free_lists = vec![Vec::new(); max_order + 1];
        free_lists[max_order].push(0); // whole memory is initially free

        BuddyAllocator {
            total_size,
            min_block,
            max_order,
            memory: vec![0u8; total_size],
            free_lists,
        }
    }

    /// Return the smallest order whose block size can accommodate `size` bytes plus header.
    fn order_for_size(&self, size: usize) -> Option<usize> {
        // Header occupies 1 byte
        let needed = size + 1;
        if needed > self.total_size {
            return None;
        }
        let mut block = self.min_block;
        let mut order = 0usize;
        while block < needed {
            block <<= 1;
            order += 1;
            if order > self.max_order {
                return None;
            }
        }
        Some(order)
    }

    /// Allocate a block of at least `size` bytes.
    /// Returns the offset (into the simulated memory) of the usable region (after header).
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        let order = self.order_for_size(size)?;
        // Find a free block at order or higher
        let mut cur_order = order;
        while cur_order <= self.max_order && self.free_lists[cur_order].is_empty() {
            cur_order += 1;
        }
        if cur_order > self.max_order {
            return None; // out of memory
        }
        // Remove a block from cur_order
        let mut block_offset = self.free_lists[cur_order].pop().unwrap();
        // Split down to desired order
        while cur_order > order {
            cur_order -= 1;
            let buddy_offset = block_offset + (self.min_block << cur_order);
            // Push the buddy into the lower order free list
            self.free_lists[cur_order].push(buddy_offset);
        }
        // Write header (order) at the start of the block
        self.memory[block_offset] = order as u8;
        // Return pointer to usable memory (skip header)
        Some(block_offset + 1)
    }

    /// Deallocate a previously allocated block given the pointer returned by `allocate`.
    /// Returns true on success, false if the pointer is invalid.
    pub fn deallocate(&mut self, ptr: usize) -> bool {
        if ptr == 0 || ptr > self.total_size {
            return false;
        }
        // Header is located one byte before the usable pointer
        let header_offset = ptr - 1;
        if header_offset >= self.total_size {
            return false;
        }
        let order = self.memory[header_offset] as usize;
        if order > self.max_order {
            return false;
        }
        let mut block_offset = header_offset;
        let mut cur_order = order;
        let mut block_size = self.min_block << cur_order;

        // Coalesce with buddy while possible
        while cur_order < self.max_order {
            let buddy_offset = block_offset ^ block_size;
            // Search for buddy in free list of current order
            if let Some(pos) = self.free_lists[cur_order]
                .iter()
                .position(|&off| off == buddy_offset)
            {
                // Remove buddy
                self.free_lists[cur_order].swap_remove(pos);
                // Determine new combined block's offset (lower address)
                block_offset = max(block_offset, buddy_offset) ^ block_size;
                cur_order += 1;
                block_size <<= 1;
            } else {
                break;
            }
        }
        // Insert the (possibly coalesced) block back into free list
        self.free_lists[cur_order].push(block_offset);
        true
    }

    /// Helper for testing: count total free bytes.
    pub fn total_free_bytes(&self) -> usize {
        let mut sum = 0usize;
        for (order, list) in self.free_lists.iter().enumerate() {
            let block_size = self.min_block << order;
            sum += block_size * list.len();
        }
        sum
    }

    /// Helper for testing: check internal consistency (all free blocks are non‑overlapping).
    pub fn check_consistency(&self) -> bool {
        let mut occupied = vec![false; self.total_size];
        for (order, list) in self.free_lists.iter().enumerate() {
            let block_size = self.min_block << order;
            for &offset in list {
                if offset + block_size > self.total_size {
                    return false;
                }
                for i in offset..offset + block_size {
                    if occupied[i] {
                        return false; // overlap
                    }
                    occupied[i] = true;
                }
            }
        }
        true
    }
}