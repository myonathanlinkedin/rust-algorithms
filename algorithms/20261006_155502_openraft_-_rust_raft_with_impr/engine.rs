use crate::types::*;
use std::cmp::min;

#[derive(Debug, Clone, PartialEq)]
pub struct RaftNode {
    pub id: NodeId,
    pub current_term: Term,
    pub voted_for: Option<NodeId>,
    pub log: Vec<LogEntry>,
    pub commit_index: usize,
    pub last_applied: usize,
    pub state: NodeState,
    pub votes_received: usize,
    pub election_elapsed: usize,
    pub election_timeout: usize,
    pub heartbeat_elapsed: usize,
    pub heartbeat_interval: usize,
    pub leader_id: Option<NodeId>,
    pub total_nodes: usize,

}

impl RaftNode {
    pub fn new(id: NodeId, total_nodes: usize, election_timeout: usize, heartbeat_interval: usize) -> Self {
        Self {
            id,
            current_term: 0,
            voted_for: None,
            log: Vec::new(),
            commit_index: 0,
            last_applied: 0,
            state: NodeState::Follower,
            votes_received: 0,
            election_elapsed: 0,
            election_timeout,
            heartbeat_elapsed: 0,
            heartbeat_interval,
            leader_id: None,
            total_nodes,
        }
    }

    pub fn tick(&mut self, cluster: &mut Cluster) {
        match self.state {
            NodeState::Follower | NodeState::Candidate => {
                self.election_elapsed += 1;
                if self.election_elapsed >= self.election_timeout {
                    self.start_election(cluster);
                }
            }
            NodeState::Leader => {
                self.heartbeat_elapsed += 1;
                if self.heartbeat_elapsed >= self.heartbeat_interval {
                    self.send_heartbeats(cluster);
                }
            }
        }
    }

    fn become_follower(&mut self, term: Term, leader: Option<NodeId>) {
        self.state = NodeState::Follower;
        self.current_term = term;
        self.voted_for = None;
        self.leader_id = leader;
        self.election_elapsed = 0;
        self.heartbeat_elapsed = 0;
    }

    fn become_candidate(&mut self) {
        self.state = NodeState::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.id);
        self.votes_received = 1;
        self.election_elapsed = 0;
    }

    fn become_leader(&mut self) {
        self.state = NodeState::Leader;
        self.leader_id = Some(self.id);
        self.heartbeat_elapsed = 0;
        // Upon election: initialize nextIndex & matchIndex (omitted for simplicity)
    }

    fn start_election(&mut self, cluster: &mut Cluster) {
        self.become_candidate();
        let last_log_index = if self.log.is_empty() { 0 } else { self.log.len() };
        let last_log_term = if self.log.is_empty() { 0 } else { self.log.last().unwrap().term };
        let rv = RequestVote {
            term: self.current_term,
            candidate_id: self.id,
            last_log_index,
            last_log_term,
        };
        cluster.broadcast_request_vote(self.id, rv);
    }

    pub fn handle_request_vote(&mut self, rv: RequestVote) -> RequestVoteResponse {
        let mut vote_granted = false;
        if rv.term < self.current_term {
            // reject
        } else {
            if rv.term > self.current_term {
                self.become_follower(rv.term, None);
            }
            let not_voted = self.voted_for.is_none() || self.voted_for == Some(rv.candidate_id);
            let up_to_date = {
                let last_index = if self.log.is_empty() { 0 } else { self.log.len() };
                let last_term = if self.log.is_empty() { 0 } else { self.log.last().unwrap().term };
                (rv.last_log_term > last_term) ||
                (rv.last_log_term == last_term && rv.last_log_index >= last_index)
            };
            if not_voted && up_to_date {
                self.voted_for = Some(rv.candidate_id);
                vote_granted = true;
                self.election_elapsed = 0;
            }
        }
        RequestVoteResponse {
            term: self.current_term,
            vote_gr
}
}
}