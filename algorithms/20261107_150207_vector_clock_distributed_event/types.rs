pub const DEFAULT_CLOCK_SIZE: usize = 0;

#[derive(Debug, Clone, PartialEq)]
pub struct VectorClock {
    /// The logical counters for each process/node.
    /// The index in the vector corresponds to the process identifier.
    pub counters: Vec<u64>,

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClockRelation {
    /// `self` happened before `other`
    Less,
    /// `self` happened after `other`
    Greater,
    /// Both clocks are identical
    Equal,
    /// Neither clock is causally related to the other
    Concurrent,
}