mod core;

use core::{InfiniteArray, cow_path_search};

fn main() {
    // Example usage
    let arr = InfiniteArray::new(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21]);
    let target = 13;
    let idx = cow_path_search(&arr, target);
    assert_eq!(idx, Some(6));
    println!("Target {} found at index {:?}", target, idx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let arr = InfiniteArray::new(vec![]);
        assert_eq!(cow_path_search(&arr, 5), None);
    }

    #[test]
    fn test_target_at_start() {
        let arr = InfiniteArray::new(vec![2, 4, 6, 8]);
        assert_eq!(cow_path_search(&arr, 2), Some(0));
    }

    #[test]
    fn test_target_at_end() {
        let arr = InfiniteArray::new(vec![1, 3, 5, 7, 9]);
        assert_eq!(cow_path_search(&arr, 9), Some(4));
    }

    #[test]
    fn test_target_not_present() {
        let arr = InfiniteArray::new(vec![1, 4, 7, 10]);
        assert_eq!(cow_path_search(&arr, 5), None);
    }

    #[test]
    fn test_target_less_than_first() {
        let arr = InfiniteArray::new(vec![10, 20, 30]);
        assert_eq!(cow_path_search(&arr, 5), None);
    }

    #[test]
    fn test_large_array() {
        let data: Vec<i32> = (0..1000).map(|x| x * 2).collect();
        let arr = InfiniteArray::new(data);
        assert_eq!(cow_path_search(&arr, 2000), Some(1000));
        assert_eq!(cow_path_search(&arr, 1999), None);
    }

    #[test]
    fn test_cyclic_behavior() {
        // Simulate cyclic by wrapping around: treat array as infinite repeating
        // For this test, we just ensure algorithm doesn't panic on repeated values
        let arr = InfiniteArray::new(vec![1, 2, 3, 1, 2, 3]);
        assert_eq!(cow_path_search(&arr, 3), Some(2));
    }
}