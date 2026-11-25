mod core;

#[cfg(test)]
mod tests {
    use super::core::bit_parallel_levenshtein;

    fn dp_distance(a: &str, b: &str) -> usize {
        let n = a.len();
        let m = b.len();
        let mut dp = vec![vec![0usize; m + 1]; n + 1];
        for i in 0..=n {
            dp[i][0] = i;
        }
        for j in 0..=m {
            dp[0][j] = j;
        }
        for i in 1..=n {
            for j in 1..=m {
                let cost = if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
                    0
                } else {
                    1
                };
                dp[i][j] = std::cmp::min(
                    std::cmp::min(dp[i - 1][j] + 1, dp[i][j - 1] + 1),
                    dp[i - 1][j - 1] + cost,
                );
            }
        }
        dp[n][m]
    }

    #[test]
    fn test_basic() {
        assert_eq!(bit_parallel_levenshtein("kitten", "sitting"), 3);
        assert_eq!(bit_parallel_levenshtein("flaw", "lawn"), 2);
        assert_eq!(bit_parallel_levenshtein("", "abc"), 3);
        assert_eq!(bit_parallel_levenshtein("abc", ""), 3);
        assert_eq!(bit_parallel_levenshtein("abc", "abc"), 0);
        assert_eq!(bit_parallel_levenshtein("abc", "abcd"), 1);
        assert_eq!(bit_parallel_levenshtein("abcd", "abc"), 1);
        assert_eq!(bit_parallel_levenshtein("a", "b"), 1);
        assert_eq!(bit_parallel_levenshtein("ab", "ba"), 2);
        assert_eq!(bit_parallel_levenshtein("abcdef", "azced"), 3);
    }

    #[test]
    fn test_random() {
        let s = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let t = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz9876543210";
        assert_eq!(bit_parallel_levenshtein(s, t), dp_distance(s, t));
    }

    #[test]
    fn test_max_length() {
        let s = "a".repeat(64);
        let t = "a".repeat(64);
        assert_eq!(bit_parallel_levenshtein(&s, &t), 0);
        let t2 = "b".repeat(64);
        assert_eq!(bit_parallel_levenshtein(&s, &t2), 64);
    }

    #[test]
    #[should_panic(expected = "Pattern length must be <= 64")]
    fn test_overflow() {
        let s = "a".repeat(65);
        let t = "a".repeat(65);
        let _ = bit_parallel_levenshtein(&s, &t);
    }
}

fn main() {
    println!("Running bit-parallel Levenshtein distance tests...");
    let dist = core::bit_parallel_levenshtein("kitten", "sitting");
    println!("Distance between 'kitten' and 'sitting': {}", dist);
}