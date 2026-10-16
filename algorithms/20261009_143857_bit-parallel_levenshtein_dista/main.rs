mod types;
mod engine;

use types::LevenshteinEngine;

fn main() {
    let engine = LevenshteinEngine::new();

    // Demo
    let dist = engine.distance("kitten", "sitting");
    println!("Distance between 'kitten' and 'sitting': {}", dist);
    assert_eq!(dist, 3);

    // Unit tests
    assert_eq!(engine.distance("", ""), 0);
    assert_eq!(engine.distance("a", ""), 1);
    assert_eq!(engine.distance("", "abc"), 3);
    assert_eq!(engine.distance("rust", "rust"), 0);

    // Test with pattern longer than 64 characters
    let long_a = "a".repeat(70);
    let long_b = "a".repeat(70);
    assert_eq!(engine.distance(&long_a, &long_b), 0);
    let long_b2 = "b".repeat(70);
    assert_eq!(engine.distance(&long_a, &long_b2), 70);

    // Additional random tests
    assert_eq!(engine.distance("abcd", "abxd"), 1);
    assert_eq!(engine.distance("abcd", "acbd"), 2);
    assert_eq!(engine.distance("abc", "yabd"), 2);
    assert_eq!(engine.distance("intention", "execution"), 5);

    println!("All tests passed.");
}