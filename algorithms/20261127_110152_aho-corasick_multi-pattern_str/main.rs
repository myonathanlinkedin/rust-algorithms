mod types;
mod engine;

use types::AhoCorasick;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let patterns = vec![
            "he".to_string(),
            "she".to_string(),
            "his".to_string(),
            "hers".to_string(),
        ];
        let ac = AhoCorasick::new(patterns);
        let text = "ushers";
        let matches = ac.search(text);
        let expected: Vec<(usize, usize)> = vec![(0, 3), (1, 3), (3, 5)];
        assert_eq!(matches, expected);
    }

    #[test]
    fn test_no_match() {
        let patterns = vec!["abc".to_string(), "def".to_string()];
        let ac = AhoCorasick::new(patterns);
        let text = "xyz";
        let matches = ac.search(text);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_overlapping() {
        let patterns = vec!["a".to_string(), "aa".to_string(), "aaa".to_string()];
        let ac = AhoCorasick::new(patterns);
        let text = "aaaa";
        let matches = ac.search(text);
        let expected: Vec<(usize, usize)> = vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 1),
            (1, 2),
            (1, 3),
            (2, 2),
            (2, 3),
        ];
        assert_eq!(matches, expected);
    }

    #[test]
    fn test_empty_text() {
        let patterns = vec!["a".to_string()];
        let ac = AhoCorasick::new(patterns);
        let text = "";
        let matches = ac.search(text);
        assert!(matches.is_empty());
    }

    #[test]
    fn test_empty_pattern() {
        let patterns = vec!["".to_string(), "a".to_string()];
        let ac = AhoCorasick::new(patterns);
        let text = "a";
        let matches = ac.search(text);
        let expected = vec![(1, 0)];
        assert_eq!(matches, expected);
    }

    #[test]
    fn test_non_ascii() {
        let patterns = vec!["a".to_string()];
        let ac = AhoCorasick::new(patterns);
        let text = "aéa";
        let matches = ac.search(text);
        let expected = vec![(0, 0), (0, 2)];
        assert_eq!(matches, expected);
    }
}

fn main() {
    let patterns = vec![
        "he".to_string(),
        "she".to_string(),
        "his".to_string(),
        "hers".to_string(),
    ];
    let ac = AhoCorasick::new(patterns);
    let text = "ushers";
    let matches = ac.search(text);
    for (pat_idx, pos) in matches {
        println!("Pattern '{}' found at position {}", ac.patterns[pat_idx], pos);
    }
}