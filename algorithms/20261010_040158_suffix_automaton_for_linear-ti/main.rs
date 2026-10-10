mod core;
use core::SuffixAutomaton;

fn main() {
    // Basic sanity checks executed at program start.
    let sam = SuffixAutomaton::build("ababa");
    assert!(sam.contains("aba"));
    assert!(sam.contains("bab"));
    assert!(sam.contains("ababa"));
    assert!(sam.contains("ba"));
    assert!(!sam.contains("c"));
    assert!(sam.contains("")); // empty pattern is always a substring

    // Edge case: empty source string.
    let empty_sam = SuffixAutomaton::build("");
    assert!(empty_sam.contains(""));
    assert!(!empty_sam.contains("a"));

    // Longer random test.
    let text = "thequickbrownfoxjumpsoverthelazydog";
    let sam2 = SuffixAutomaton::build(text);
    assert!(sam2.contains("quick"));
    assert!(sam2.contains("brown"));
    assert!(sam2.contains("lazy"));
    assert!(!sam2.contains("quack"));
    assert!(!sam2.contains("thethe"));
    println!("All assertions passed.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_substrings() {
        let sam = SuffixAutomaton::build("abcab");
        let substrings = vec!["a", "ab", "abc", "bca", "cab", "abcab", ""];
        for sub in substrings {
            assert!(sam.contains(sub), "should contain '{}'", sub);
        }
        let non_substrings = vec!["ac", "ba", "cabc", "d"];
        for sub in non_substrings {
            assert!(!sam.contains(sub), "should not contain '{}'", sub);
        }
    }

    #[test]
    fn test_repeated_patterns() {
        let sam = SuffixAutomaton::build("aaaaa");
        assert!(sam.contains("a"));
        assert!(sam.contains("aa"));
        assert!(sam.contains("aaa"));
        assert!(sam.contains("aaaa"));
        assert!(sam.contains("aaaaa"));
        assert!(!sam.contains("aaaaaa"));
    }

    #[test]
    fn test_unicode_handling() {
        let sam = SuffixAutomaton::build("😀😃😄😁");
        assert!(sam.contains("😀"));
        assert!(sam.contains("😃😄"));
        assert!(sam.contains("😁"));
        assert!(!sam.contains("😆"));
    }

    #[test]
    fn test_empty_source() {
        let sam = SuffixAutomaton::build("");
        assert!(sam.contains(""));
        assert!(!sam.contains("a"));
    }
}