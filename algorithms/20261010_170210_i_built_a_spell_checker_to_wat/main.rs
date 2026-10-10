mod core;
use core::Trie;

fn main() {
    let mut dict = Trie::new();
    dict.insert("hello");
    dict.insert("world");
    dict.insert("help");
    dict.insert("held");

    assert!(dict.contains("hello"));
    assert!(!dict.contains("hell"));
    let sugg = dict.suggestions("helo");
    assert!(sugg.contains(&"hello".to_string()));
    println!("Suggestions for 'helo': {:?}", sugg);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut trie = Trie::new();
        let words = ["rust", "rustacean", "trust", "dust"];
        for w in &words {
            trie.insert(w);
        }
        for w in &words {
            assert!(trie.contains(w));
        }
        assert!(!trie.contains("rusty"));
        assert!(!trie.contains(""));
    }

    #[test]
    fn test_suggestions_basic() {
        let mut trie = Trie::new();
        let dict = ["cat", "bat", "rat", "drat", "dart"];
        for w in &dict {
            trie.insert(w);
        }
        let sugg = trie.suggestions("dat");
        let expected = ["bat", "cat", "rat", "dart"];
        for e in &expected {
            assert!(sugg.contains(&e.to_string()));
        }
        assert!(!sugg.contains(&"drat".to_string()));
    }

    #[test]
    fn test_suggestions_no_match() {
        let mut trie = Trie::new();
        trie.insert("example");
        let sugg = trie.suggestions("zzz");
        assert!(sugg.is_empty());
    }

    #[test]
    fn test_edge_cases() {
        let mut trie = Trie::new();
        trie.insert("");
        assert!(trie.contains(""));
        let sugg = trie.suggestions("");
        assert!(sugg.is_empty());
    }
}