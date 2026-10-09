use crate::types::SubmodularOracle;
use std::cmp::Ordering;

/// Naïve local‑search minimizer that uses only the comparison oracle.
///
/// The algorithm starts from the empty set, greedily adds elements that do not increase
/// the function value, and then repeatedly tries to remove elements that strictly improve
/// the value.  It terminates with a locally minimal set, which for many symmetric
/// submodular functions coincides with the global minimum.
pub fn minimize(universe: usize, oracle: &impl SubmodularOracle) -> Vec<usize> {
    // Start with the empty set.
    let mut current: Vec<usize> = Vec::new();

    // Greedy addition phase.
    for v in 0..universe {
        let mut candidate = current.clone();
        candidate.push(v);
        candidate.sort_unstable(); // keep deterministic order
        if oracle.compare(&candidate, &current) != Ordering::Greater {
            current = candidate;
        }
    }

    // Removal phase: keep removing any element that strictly improves the value.
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..current.len() {
            let mut candidate = current.clone();
            candidate.remove(i);
            if oracle.compare(&candidate, &current) == Ordering::Less {
                current = candidate;
                changed = true;
                break;
            }
        }
    }

    current
}