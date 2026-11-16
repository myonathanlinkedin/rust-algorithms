mod core;
use core::{Trie};

fn main() {
    // Basic sanity checks executed at runtime
    let mut trie = Trie::new();
    trie.insert("apple", 5);
    trie.insert("app", 10);
    trie.insert("application", 3);
    trie.insert("apex", 7);
    trie.insert("banana", 4);
    trie.insert("band", 2);
    trie.insert("bandana", 6);
    trie.insert("bandit", 1);
    trie.insert("cat", 8);
    trie.insert("cater", 2);
    trie.insert("cattle", 1);
    // Prefix "ap" should return top suggestions among "app", "apex", "apple", "application"
    let sug = trie.suggestions("ap");
    assert_eq!(sug, vec!["app".to_string(), "apex".to_string(), "apple".to_string(), "application".to_string()]);
    // Prefix "ban" should return "bandana", "banana", "band", "bandit"
    let sug_ban = trie.suggestions("ban");
    assert_eq!(sug_ban, vec!["bandana".to_string(), "banana".to_string(), "band".to_string(), "bandit".to_string()]);
    // Prefix with no matches yields empty vector
    assert!(trie.suggestions("xyz").is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_insert_and_suggest() {
        let mut trie = Trie::new();
        trie.insert("dog", 4);
        trie.insert("deer", 2);
        trie.insert("deal", 5);
        trie.insert("doom", 1);
        let s = trie.suggestions("de");
        assert_eq!(s, vec!["deal".to_string(), "deer".to_string()]);
    }

    #[test]
    fn test_top_limit_ten() {
        let mut trie = Trie::new();
        // Insert 12 words with decreasing frequencies
        for i in 0..12 {
            let word = format!("word{}", i);
            trie.insert(&word, 12 - i);
        }
        let all = trie.suggestions("");
        assert_eq!(all.len(), 10);
        // The highest frequency word should be first
        assert_eq!(all[0], "word0".to_string());
        // The 10th should be "word9"
        assert_eq!(all[9], "word9".to_string());
        // "word10" and "word11" must be absent
        assert!(!all.contains(&"word10".to_string()));
        assert!(!all.contains(&"word11".to_string()));
    }

    #[test]
    fn test_update_existing_word() {
        let mut trie = Trie::new();
        trie.insert("alpha", 3);
        trie.insert("alpha", 10); // overwrite with higher frequency
        let s = trie.suggestions("a");
        assert_eq!(s[0], "alpha".to_string());
        // Ensure frequency used for ordering is the latest one
        trie.insert("beta", 9);
        let s2 = trie.suggestions("a");
        assert_eq!(s2[0], "alpha".to_string());
        assert_eq!(s2[1], "beta".to_string());
    }

    #[test]
    fn test_prefix_not_present() {
        let mut trie = Trie::new();
        trie.insert("hello", 5);
        assert!(trie.suggestions("world").is_empty());
    }
}