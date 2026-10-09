use crate::types::LevenshteinEngine;

impl LevenshteinEngine {
    /// Compute the Levenshtein distance between two strings.
    /// Uses Myers' bit‑parallel algorithm for patterns up to 64 characters.
    /// Falls back to classic dynamic programming for longer patterns.
    pub fn distance(&self, a: &str, b: &str) -> u32 {
        // Choose the shorter string as the pattern to keep bit‑vector size minimal.
        let (pattern, text) = if a.len() <= b.len() { (a, b) } else { (b, a) };
        let m = pattern.len();

        // Handle trivial cases.
        if m == 0 {
            return text.len() as u32;
        }

        // For patterns longer than 64, use the DP fallback.
        if m > 64 {
            return self.distance_dp(pattern, text);
        }

        // Pre‑compute the Peq table: for each ASCII character, a bitmask
        // indicating the positions of that character in the pattern.
        let mut peq: [u64; 256] = [0; 256];
        for (i, ch) in pattern.chars().enumerate() {
            let idx = ch as usize;
            peq[idx] |= 1u64 << i;
        }

        // Myers' algorithm state.
        let mut vp: u64 = !0;
        let mut vn: u64 = 0;
        let mut score: u32 = m as u32;
        let mask: u64 = 1u64 << (m - 1);

        for ch in text.chars() {
            let eq = peq[ch as usize];
            let x = eq | vn;
            let d0 = (((x & vp).wrapping_add(vp)) ^ vp) | x;
            let hp = vn | !(d0 | vp);
            let hn = vp & d0;

            if (hp & mask) != 0 {
                score += 1;
            }
            if (hn & mask) != 0 {
                score -= 1;
            }

            vp = (hn << 1) | !(d0 | ((hp << 1) | 1));
            vn = (hp << 1) & d0;
        }

        score
    }

    /// Classic dynamic programming implementation for arbitrary pattern lengths.
    fn distance_dp(&self, a: &str, b: &str) -> u32 {
        let m = a.len();
        let n = b.len();
        let mut dp = vec![vec![0u32; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i as u32;
        }
        for j in 0..=n {
            dp[0][j] = j as u32;
        }

        for i in 1..=m {
            let ca = a.as_bytes()[i - 1];
            for j in 1..=n {
                let cb = b.as_bytes()[j - 1];
                let cost = if ca == cb { 0 } else { 1 };
                let del = dp[i - 1][j] + 1;
                let ins = dp[i][j - 1] + 1;
                let sub = dp[i - 1][j - 1] + cost;
                dp[i][j] = del.min(ins).min(sub);
            }
        }

        dp[m][n]
    }
}