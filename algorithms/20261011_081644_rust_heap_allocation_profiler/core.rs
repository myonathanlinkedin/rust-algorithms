use std::collections::HashMap;

/// Statistics aggregated per span.
#[derive(Debug, Clone, PartialEq)]
pub struct SpanStat {
    /// Total bytes allocated for this span.
    pub total: usize,
    /// Number of allocation events for this span.
    pub count: usize,

}

/// A snapshot of the profiler at a given moment.
/// Stores total allocated bytes per span.
#[derive(Debug, Clone, PartialEq)]
pub struct Snapshot {
    pub span_totals: HashMap<String, usize>,

}

impl Snapshot {
    /// Compute a regression report comparing `self` (old) with `newer`.
    /// A regression is reported when the newer total exceeds the older total.
    pub fn diff(&self, newer: &Snapshot) -> RegressionReport {
        let mut increased = Vec::new();
        for (span, &new_total) in newer.span_totals.iter() {
            let old_total = self.span_totals.get(span).copied().unwrap_or(0);
            if new_total > old_total {
                increased.push((span.clone(), old_total, new_total));
            }
        }
        // Sort by magnitude of increase descending for deterministic output.
        increased.sort_by(|a, b| (b.2 - b.1).cmp(&(a.2 - a.1)));
        RegressionReport { increased }
    }
}

/// Report describing spans whose allocation grew between two snapshots.
#[derive(Debug, Clone, PartialEq)]
pub struct RegressionReport {
    /// (span, old_total, new_total) where new_total > old_total
    pub increased: Vec<(String, usize, usize)>,

}

impl RegressionReport {
    /// Returns true if any regression was detected.
    pub fn has_regressions(&self) -> bool {
        !self.increased.is_empty()
    }
}

/// Core profiler that records allocation events with span awareness.
#[derive(Debug, Default)]
pub struct Profiler {
    /// All allocation events recorded (kept for possible future extensions).
    pub allocations: Vec<Allocation>,
    /// Aggregated statistics per span.
    pub span_stats: HashMap<String, SpanStat>,

}

/// Single allocation record.
#[derive(Debug, Clone, PartialEq)]
pub struct Allocation {
    pub size: usize,
    pub span: String,

}

impl Profiler {
    /// Create a new, empty profiler.
    pub fn new() -> Self {
        Self {
            allocations: Vec::new(),
            span_stats: HashMap::new(),
        }
    }

    /// Record an allocation of `size` bytes associated with `span`.
    /// `span` is any identifier (e.g., function name, code region).
    pub fn record_allocation(&mut self, size: usize, span: &str) {
        let span_key = span.to_string();
        self.allocations.push(Allocation {
            size,
            span: span_key.clone(),
        });
        let entry = self.span_stats.entry(span_key).or_insert(SpanStat {
            total: 0,
            count: 0,
        });
        entry.total += size;
        entry.count += 1;
    }

    /// Return a snapshot of current aggregated totals per span.
    pub fn snapshot(&self) -> Snapshot {
        let mut span_totals = HashMap::new();
        for (span, stat) in self.span_stats.iter() {
            span_totals.insert(span.clone(), stat.total);
        }
        Snapshot { span_totals }
    }

    /// Return the top `n` spans sorted by total allocated bytes descending.
    pub fn top_spans(&self, n: usize) -> Vec<(String, usize)> {
        let mut vec: Vec<(String, usize)> = self
            .span_stats
            .iter()
            .map(|(span, stat)| (span.clone(), stat.total))
            .collect();
        vec.sort_by(|a, b| b.1.cmp(&a.1));
        vec.truncate(n);
        vec
    }

    /// Retrieve the aggregated `SpanStat` for a given span, if present.
    pub fn get_span_stat(&self, span: &str) -> Option<SpanStat> {
        self.span_stats.get(span).cloned()
    }
}