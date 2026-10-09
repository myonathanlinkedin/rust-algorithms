use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq)]
pub struct InfiniteArray {
    pub data: Vec<i32>,

}

impl InfiniteArray {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn get(&self, idx: usize) -> Option<i32> {
        self.data.get(idx).copied()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub fn cow_path_search(arr: &InfiniteArray, target: i32) -> Option<usize> {
    if arr.len() == 0 {
        return None;
    }
    // If target is less than first element
    if let Some(first) = arr.get(0) {
        if target < first {
            return None;
        }
    }

    let mut bound = 1usize;
    while let Some(val) = arr.get(bound) {
        if val >= target {
            break;
        }
        bound <<= 1;
    }

    let mut low = bound >> 1;
    let mut high = bound;
    while low <= high {
        let mid = low + (high - low) / 2;
        match arr.get(mid) {
            Some(val) if val == target => return Some(mid),
            Some(val) if val < target => low = mid + 1,
            _ => {
                if mid == 0 {
                    break;
                }
                high = mid - 1;
            }
        }
    }
    None
}