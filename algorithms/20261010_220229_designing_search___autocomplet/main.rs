mod core;
use core::InvertedIndex;

fn main() {
    println!("InvertedIndex demo");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_search() {
        let mut idx = InvertedIndex::new();
        idx.add_document(1, "hello world hello");
        idx.add_document(2, "hello rust");
        idx.add_document(3, "world of rust");

        assert_eq!(idx.search("hello"), vec![1, 2]);
        assert_eq!(idx.search("world"), vec![1, 3]);
        assert_eq!(idx.search("rust"), vec![2, 3]);
        assert!(idx.search("nonexistent").is_empty());
    }

    #[test]
    fn test_autocomplete_basic() {
        let mut idx = InvertedIndex::new();
        idx.add_document(1, "hello world hello");
        idx.add_document(2, "hello rust");
        idx.add_document(3, "world of rust");

        let res_he = idx.autocomplete("he");
        assert_eq!(res_he, vec![("hello".to_string(), 3)]);

        let res_wo = idx.autocomplete("wo");
        assert_eq!(res_wo, vec![("world".to_string(), 2)]);

        let res_r = idx.autocomplete("r");
        assert_eq!(res_r, vec![("rust".to_string(), 2)]);
    }

    #[test]
    fn test_autocomplete_full() {
        let mut idx = InvertedIndex::new();
        idx.add_document(1, "hello world hello");
        idx.add_document(2, "hello rust");
        idx.add_document(3, "world of rust");

        let res = idx.autocomplete("");
        let expected = vec![
            ("hello".to_string(), 3),
            ("world".to_string(), 2),
            ("rust".to_string(), 2),
            ("of".to_string(), 1),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_ranking_order() {
        let mut idx = InvertedIndex::new();
        idx.add_document(1, "a b c");
        idx.add_document(2, "a a b");
        idx.add_document(3, "a a a");

        let res = idx.autocomplete("");
        let expected = vec![
            ("a".to_string(), 6),
            ("b".to_string(), 2),
            ("c".to_string(), 1),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_edge_cases() {
        let mut idx = InvertedIndex::new();
        idx.add_document(1, "");
        idx.add_document(2, "   ");
        idx.add_document(3, "test");

        assert!(idx.search("test").contains(&3));
        assert!(idx.autocomplete("").len() == 1);
        assert_eq!(idx.autocomplete("x").len(), 0);
    }
}