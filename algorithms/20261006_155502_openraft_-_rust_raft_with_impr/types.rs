pub type Term = u64;
pub type NodeId = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub term: Term,
    pub command: String,

}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeState {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestVote {
    pub term: Term,
    pub candidate_id: NodeId,
    pub last_log_index: usize,
    pub last_log_term: Term,

}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestVoteResponse {
    pub term: Term,
    pub vote_granted: bool,

}

#[derive(Debug, Clone, PartialEq)]
pub struct AppendEntries {
    pub term: Term,
    pub leader_id: NodeId,
    pub prev_log_index: usize,
    pub prev_log_term: Term,
    pub entries: Vec<LogEntry>,
    pub leader_commit: usize,

}

#[derive(Debug, Clone, PartialEq)]
pub struct AppendEntriesResponse {
    pub term: Term,
    pub success: bool,

}