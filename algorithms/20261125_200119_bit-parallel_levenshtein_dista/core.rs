use std::cmp::min;

pub fn bit_parallel_levenshtein(s: &str, t: &str) -> usize {
    let m = t.len();
    if m == 0 {
        return s.len();
    }
    if s.is_empty() {
        return m;
    }
    assert!(m <= 64, "Pattern length must be <= 64 for this implementation");
    let mut bitmask: [u64; 256] = [0; 256];
    for (i, ch) in t.chars().enumerate() {
        let idx = ch as usize;
        bitmask[idx] |= 1u64 << i;
    }
    let mut vp: u64 = !0;
    let mut vn: u64 = 0;
    let mut score: i32 = m as i32;
    let mask: u64 = 1u64 << (m - 1);
    for ch in s.chars() {
        let pm = bitmask[ch as usize];
        let x = pm | vn;
        let d0 = (((x & vp).wrapping_add(vp)) ^ vp) | x;
        let hp = vn | !(d0 | vp);
        let hn = vp & d0;
        vp = (hn << 1) | !(d0 | hp);
        vn = (hp << 1) & d0;
        if (d0 & mask) != 0 {
            score += 1;
        } else if (d0 & mask) == 0 {
            score -= 1;
        }
    }
    score as usize
}