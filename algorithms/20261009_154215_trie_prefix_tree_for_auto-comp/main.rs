mod types;
mod engine;

use crate::types::Trie;

fn main() {
    // Basic sanity checks.
    let mut trie = Trie::new();

    // Insert words with varying frequencies.
    trie.insert("apple");
    trie.insert("app");
    trie.insert("application");
    trie.insert("app");
    trie.insert("apex");
    trie.insert("banana");
    trie.insert("band");
    trie.insert("band");
    trie.insert("bandana");
    trie.insert("bandana");
    trie.insert("bandana");
    trie.insert("bandit");
    trie.insert("bandit");
    trie.insert("bandit");
    trie.insert("bandit");

    // Test autocomplete for prefix "app" with limit 3.
    let completions = trie.autocomplete("app", 3);
    let expected = vec![
        ("app".to_string(), 2),          // frequency 2
        ("apple".to_string(), 1),        // frequency 1
        ("application".to_string(), 1), // frequency 1
    ];
    assert_eq!(completions, expected);

    // Test autocomplete for prefix "ban" with limit 2.
    let completions = trie.autocomplete("ban", 2);
    let expected = vec![
        ("bandit".to_string(), 4), // highest frequency
        ("bandana".to_string(), 3), // second highest
    ];
    assert_eq!(completions, expected);

    // Test autocomplete for a non‑existent prefix.
    let completions = trie.autocomplete("xyz", 5);
    assert!(completions.is_empty());

    // Test limit larger than available suggestions.
    let completions = trie.autocomplete("ap", 10);
    let expected = vec![
        ("app".to_string(), 2),
        ("apple".to_string(), 1),
        ("application".to_string(), 1),
        ("apex".to_string(), 1),
    ];
    assert_eq!(completions, expected);

    // Edge case: empty prefix should return top words overall.
    let completions = trie.autocomplete("", 4);
    let expected = vec![
        ("bandit".to_string(), 4),
        ("bandana".to_string(), 3),
        ("app".to_string(), 2),
        ("apple".to_string(), 1), // lexical order among same frequency
    ];
    assert_eq!(completions, expected);
}