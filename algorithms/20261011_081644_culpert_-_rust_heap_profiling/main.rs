mod core;

use core::{Profiler, RegressionReport, Snapshot};

fn main() {
    // Simple sanity checks executed at program start.
    let mut prof = Profiler::new();
    prof.record_allocation(100, "init");
    prof.record_allocation(200, "load");
    prof.record_allocation(50, "init");
    let snap1 = prof.snapshot();

    prof.record_allocation(150, "load");
    prof.record_allocation(80, "process");
    let snap2 = prof.snapshot();

    let report = snap1.diff(&snap2);
    assert!(report.has_regressions());
    // Expect "load" to have increased from 200 to 350, and "process" from 0 to 80.
    let mut expected = vec![
        ("load".to_string(), 200, 350),
        ("process".to_string(), 0, 80),
    ];
    expected.sort_by(|a, b| (b.2 - b.1).cmp(&(a.2 - a.1)));
    assert_eq!(report.increased, expected);
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_recording() {
        let mut prof = Profiler::new();
        prof.record_allocation(64, "foo");
        prof.record_allocation(128, "foo");
        prof.record_allocation(32, "bar");

        let foo_stat = prof.get_span_stat("foo").expect("foo stat missing");
        assert_eq!(foo_stat.total, 192);
        assert_eq!(foo_stat.count, 2);

        let bar_stat = prof.get_span_stat("bar").expect("bar stat missing");
        assert_eq!(bar_stat.total, 32);
        assert_eq!(bar_stat.count, 1);
    }

    #[test]
    fn test_snapshot_and_diff() {
        let mut prof = Profiler::new();
        prof.record_allocation(10, "a");
        prof.record_allocation(20, "b");
        let snap_a = prof.snapshot();

        prof.record_allocation(15, "a"); // a total becomes 25
        prof.record_allocation(5, "c");  // new span
        let snap_b = prof.snapshot();

        let report = snap_a.diff(&snap_b);
        assert!(report.has_regressions());
        // Expected increases: a from 10 to 25, c from 0 to 5, b unchanged.
        let mut expected = vec![
            ("a".to_string(), 10, 25),
            ("c".to_string(), 0, 5),
        ];
        expected.sort_by(|a, b| (b.2 - b.1).cmp(&(a.2 - a.1)));
        assert_eq!(report.increased, expected);
    }

    #[test]
    fn test_top_spans_ordering() {
        let mut prof = Profiler::new();
        prof.record_allocation(300, "alpha");
        prof.record_allocation(150, "beta");
        prof.record_allocation(200, "gamma");
        prof.record_allocation(50, "beta"); // beta total 200

        let top2 = prof.top_spans(2);
        assert_eq!(top2.len(), 2);
        assert_eq!(top2[0], ("alpha".to_string(), 300));
        // gamma and beta both have 200; deterministic order by span name after total tie.
        // Since we sort only by total, the order of equal totals is unspecified;
        // we assert that one of them appears second.
        let second = &top2[1];
        let ok = (second == &("beta".to_string(), 200)) || (second == &("gamma".to_string(), 200));
        assert!(ok);
    }

    #[test]
    fn test_no_regression() {
        let mut prof = Profiler::new();
        prof.record_allocation(100, "x");
        let snap1 = prof.snapshot();

        // No new allocations.
        let snap2 = prof.snapshot();

        let report = snap1.diff(&snap2);
        assert!(!report.has_regressions());
        assert!(report.increased.is_empty());
    }
}