mod core;
use core::AhoCorasick;

fn main() {
    // Example usage
    let patterns = ["he", "she", "his", "hers"];
    let ac = AhoCorasick::new(&patterns);
    let text = "ahishers";
    let matches = ac.search(text);
    for (pos, pat_idx) in matches {
        println!("Match '{}' at position {}", patterns[pat_idx], pos);
    }

    // Run tests
    test_basic();
    test_overlapping();
    test_no_match();
    test_empty_pattern();
    test_single_char();
    println!("All tests passed.");
}

fn test_basic() {
    let patterns = ["he", "she", "his", "hers"];
    let ac = AhoCorasick::new(&patterns);
    let text = "ahishers";
    let matches = ac.search(text);
    let expected = vec![(1, 2), (3, 0), (4, 3), (5, 1)];
    assert_eq!(matches, expected);
}

fn test_overlapping() {
    let patterns = ["a", "aa", "aaa"];
    let ac = AhoCorasick::new(&patterns);
    let text = "aaaa";
    let matches = ac.search(text);
    // Expected matches: positions for each pattern
    let mut expected = Vec::new();
    for i in 0..4 {
        expected.push((i, 0)); // "a"
    }
    for i in 0..3 {
        expected.push((i, 1)); // "aa"
    }
    for i in 0..2 {
        expected.push((i, 2)); // "aaa"
    }
    assert_eq!(matches, expected);
}

fn test_no_match() {
    let patterns = ["xyz", "abc"];
    let ac = AhoCorasick::new(&patterns);
    let text = "hello world";
    let matches = ac.search(text);
    assert!(matches.is_empty());
}

fn test_empty_pattern() {
    let patterns: [&str; 0] = [];
    let ac = AhoCorasick::new(&patterns);
    let text = "anything";
    let matches = ac.search(text);
    assert!(matches.is_empty());
}

fn test_single_char() {
    let patterns = ["a", "b", "c"];
    let ac = AhoCorasick::new(&patterns);
    let text = "abcabc";
    let matches = ac.search(text);
    let expected = vec![
        (0, 0), (1, 1), (2, 2),
        (3, 0), (4, 1), (5, 2),
    ];
    assert_eq!(matches, expected);
}