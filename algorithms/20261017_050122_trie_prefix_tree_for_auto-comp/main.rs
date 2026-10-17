mod core;
use core::Trie;

fn main() {
    // Basic sanity checks executed at runtime.
    let mut trie = Trie::new();
    trie.insert("rust", 10);
    trie.insert("rustacean", 5);
    trie.insert("run", 7);
    trie.insert("runner", 3);
    trie.insert("rune", 4);

    // Autocomplete "ru" with limit 3.
    let suggestions = trie.autocomplete("ru", 3);
    assert_eq!(suggestions, vec![
        ("rust".to_string(), 10),
        ("run".to_string(), 7),
        ("rune".to_string(), 4),
    ]);

    // Updating frequency should keep the higher value.
    trie.insert("run", 12);
    let suggestions = trie.autocomplete("ru", 2);
    assert_eq!(suggestions, vec![
        ("run".to_string(), 12),
        ("rust".to_string(), 10),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_insertion_and_lookup() {
        let mut trie = Trie::new();
        trie.insert("apple", 5);
        trie.insert("app", 3);
        trie.insert("application", 2);
        trie.insert("ape", 4);
        trie.insert("banana", 1);

        let result = trie.autocomplete("app", 10);
        let expected = vec![
            ("apple".to_string(), 5),
            ("app".to_string(), 3),
            ("application".to_string(), 2),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_prefix_not_present() {
        let mut trie = Trie::new();
        trie.insert("cat", 2);
        trie.insert("dog", 3);
        assert!(trie.autocomplete("z", 5).is_empty());
    }

    #[test]
    fn test_limit_truncation() {
        let mut trie = Trie::new();
        trie.insert("a", 1);
        trie.insert("ab", 2);
        trie.insert("abc", 3);
        trie.insert("abcd", 4);
        let result = trie.autocomplete("a", 2);
        let expected = vec![
            ("abcd".to_string(), 4),
            ("abc".to_string(), 3),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_zero_limit_returns_empty() {
        let mut trie = Trie::new();
        trie.insert("test", 1);
        assert!(trie.autocomplete("test", 0).is_empty());
    }

    #[test]
    fn test_frequency_update_keeps_max() {
        let mut trie = Trie::new();
        trie.insert("hello", 5);
        trie.insert("hello", 3);
        assert_eq!(trie.autocomplete("he", 1), vec![("hello".to_string(), 5)]);
        trie.insert("hello", 10);
        assert_eq!(trie.autocomplete("he", 1), vec![("hello".to_string(), 10)]);
    }

    #[test]
    fn test_lexicographic_tie_break() {
        let mut trie = Trie::new();
        trie.insert("bat", 2);
        trie.insert("ball", 2);
        trie.insert("cat", 1);
        let result = trie.autocomplete("b", 5);
        let expected = vec![
            ("ball".to_string(), 2),
            ("bat".to_string(), 2),
        ];
        assert_eq!(result, expected);
    }
}