mod core;
use core::LRUCache;

fn main() {
    // Simple sanity checks
    let mut cache = LRUCache::new(2);
    cache.put(1, 10);
    cache.put(2, 20);
    assert_eq!(cache.get(1), 10);
    cache.put(3, 30); // evicts key 2
    assert_eq!(cache.get(2), -1);
    cache.put(4, 40); // evicts key 1
    assert_eq!(cache.get(1), -1);
    assert_eq!(cache.get(3), 30);
    assert_eq!(cache.get(4), 40);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_operations() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3); // evicts key 2
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn update_existing_key() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(1, 10); // update value
        assert_eq!(cache.get(1), 10);
        cache.put(2, 2);
        cache.put(3, 3); // evicts key 2 (least recently used)
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(1), 10);
        assert_eq!(cache.get(3), 3);
    }

    #[test]
    fn capacity_one() {
        let mut cache = LRUCache::new(1);
        cache.put(1, 1);
        assert_eq!(cache.get(1), 1);
        cache.put(2, 2); // evicts key 1
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(2), 2);
    }

    #[test]
    fn get_nonexistent() {
        let mut cache = LRUCache::new(3);
        assert_eq!(cache.get(42), -1);
        cache.put(42, 100);
        assert_eq!(cache.get(42), 100);
        assert_eq!(cache.get(7), -1);
    }

    #[test]
    fn repeated_gets_affect_order() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1); // 1 becomes most recent
        cache.put(3, 3); // evicts 2
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(1), 1);
        assert_eq!(cache.get(3), 3);
    }
}