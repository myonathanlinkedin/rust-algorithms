mod types;
mod engine;

use engine::SuffixAutomaton;

fn main() {
    // Basic sanity checks.
    let sam = SuffixAutomaton::build("");
    assert!(sam.contains("")); // empty substring always present
    assert!(!sam.contains("a"));

    // Build automaton for a non‑trivial string.
    let text = "ababa";
    let sam = SuffixAutomaton::build(text);

    // Positive cases.
    assert!(sam.contains("a"));
    assert!(sam.contains("b"));
    assert!(sam.contains("ab"));
    assert!(sam.contains("ba"));
    assert!(sam.contains("aba"));
    assert!(sam.contains("bab"));
    assert!(sam.contains("ababa"));
    assert!(sam.contains("")); // empty substring

    // Negative cases.
    assert!(!sam.contains("c"));
    assert!(!sam.contains("aa"));
    assert!(!sam.contains("bb"));
    assert!(!sam.contains("abb"));
    assert!(!sam.contains("babaac"));

    // Additional edge cases.
    let sam2 = SuffixAutomaton::build("aaaaa");
    assert!(sam2.contains("a"));
    assert!(sam2.contains("aa"));
    assert!(sam2.contains("aaa"));
    assert!(sam2.contains("aaaa"));
    assert!(sam2.contains("aaaaa"));
    assert!(!sam2.contains("b"));
    assert!(!sam2.contains("ab"));

    // Verify that building a longer random string works without panic.
    let long = "thequickbrownfoxjumpsoverthelazydog";
    let sam_long = SuffixAutomaton::build(long);
    assert!(sam_long.contains("quick"));
    assert!(sam_long.contains("lazy"));
    assert!(!sam_long.contains("quack"));
    assert!(!sam_long.contains("doge"));
}