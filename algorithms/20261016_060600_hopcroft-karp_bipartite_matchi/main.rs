mod core;
use core::HopcroftKarp;

fn main() {
    // Simple sanity check executed at runtime.
    let mut hk = HopcroftKarp::new(2, 2);
    hk.add_edge(0, 0);
    hk.add_edge(1, 1);
    let (size, left_match, right_match) = hk.max_matching();
    assert_eq!(size, 2);
    assert_eq!(left_match[0], Some(0));
    assert_eq!(left_match[1], Some(1));
    assert_eq!(right_match[0], Some(0));
    assert_eq!(right_match[1], Some(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph() {
        let hk = HopcroftKarp::new(0, 0);
        let (size, left_match, right_match) = hk.max_matching();
        assert_eq!(size, 0);
        assert!(left_match.is_empty());
        assert!(right_match.is_empty());
    }

    #[test]
    fn no_edges() {
        let hk = HopcroftKarp::new(3, 3);
        let (size, left_match, right_match) = hk.max_matching();
        assert_eq!(size, 0);
        assert_eq!(left_match, vec![None, None, None]);
        assert_eq!(right_match, vec![None, None, None]);
    }

    #[test]
    fn perfect_matching_small() {
        let mut hk = HopcroftKarp::new(3, 3);
        hk.add_edge(0, 0);
        hk.add_edge(0, 1);
        hk.add_edge(1, 1);
        hk.add_edge(1, 2);
        hk.add_edge(2, 0);
        hk.add_edge(2, 2);
        let (size, left_match, right_match) = hk.max_matching();
        assert_eq!(size, 3);
        // Verify that each left vertex is matched to a distinct right vertex.
        let mut used = vec![false; 3];
        for &opt in &left_match {
            let v = opt.expect("left vertex should be matched");
            assert!(!used[v], "right vertex matched twice");
            used[v] = true;
        }
        // Symmetric check for right side.
        let mut used_left = vec![false; 3];
        for &opt in &right_match {
            let u = opt.expect("right vertex should be matched");
            assert!(!used_left[u], "left vertex matched twice");
            used_left[u] = true;
        }
    }

    #[test]
    fn imperfect_matching() {
        // Left side larger than right side; maximum matching size should be 2.
        let mut hk = HopcroftKarp::new(4, 2);
        hk.add_edge(0, 0);
        hk.add_edge(1, 0);
        hk.add_edge(2, 1);
        hk.add_edge(3, 1);
        let (size, left_match, right_match) = hk.max_matching();
        assert_eq!(size, 2);
        // Exactly two right vertices must be matched.
        assert_eq!(right_match.iter().filter(|m| m.is_some()).count(), 2);
        // No left vertex is matched to a non‑existent right vertex.
        for opt in left_match {
            if let Some(v) = opt {
                assert!(v < 2);
            }
        }
    }

    #[test]
    fn duplicate_edges_ignored() {
        let mut hk = HopcroftKarp::new(2, 2);
        hk.add_edge(0, 0);
        hk.add_edge(0, 0); // duplicate
        hk.add_edge(1, 1);
        hk.add_edge(1, 1); // duplicate
        let (size, left_match, right_match) = hk.max_matching();
        assert_eq!(size, 2);
        assert_eq!(left_match[0], Some(0));
        assert_eq!(left_match[1], Some(1));
        assert_eq!(right_match[0], Some(0));
        assert_eq!(right_match[1], Some(1));
    }

    #[test]
    fn larger_random_like() {
        // Construct a bipartite graph where left vertices 0..5 connect to right vertices 0..4
        // in a pattern that yields a matching of size 4.
        let mut hk = HopcroftKarp::new(6, 5);
        hk.add_edge(0, 0);
        hk.add_edge(0, 1);
        hk.add_edge(1, 1);
        hk.add_edge(1, 2);
        hk.add_edge(2, 2);
        hk.add_edge(2, 3);
        hk.add_edge(3, 3);
        hk.add_edge(3, 4);
        hk.add_edge(4, 0);
        hk.add_edge(4, 4);
        hk.add_edge(5, 2);
        let (size, left_match, right_match) = hk.max_matching();
        assert_eq!(size, 4);
        // Ensure each matched right vertex is unique.
        let mut seen = vec![false; 5];
        for opt in right_match.iter().filter_map(|&x| x) {
            assert!(!seen[opt], "right vertex matched twice");
            seen[opt] = true;
        }
    }
}