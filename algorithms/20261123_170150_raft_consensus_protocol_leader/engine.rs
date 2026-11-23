use crate::types::{NodeId, Term, Role, Message, VoteRequest, VoteResponse};

pub struct RaftNode {
    pub id: NodeId,
    pub current_term: Term,
    pub voted_for: Option<NodeId>,
    pub role: Role,
    pub peers: Vec<NodeId>,
    pub votes_received: usize,
    pub election_timeout: usize,
    pub timeout_counter: usize,

}

impl RaftNode {
    pub fn new(id: NodeId, peers: Vec<NodeId>, election_timeout: usize) -> Self {
        Self {
            id,
            current_term: 0,
            voted_for: None,
            role: Role::Follower,
            peers,
            votes_received: 0,
            election_timeout,
            timeout_counter: 0,
        }
    }

    /// Advance one logical tick. Returns messages that this node wants to send.
    pub fn tick(&mut self) -> Vec<(NodeId, Message)> {
        self.timeout_counter += 1;
        if self.timeout_counter >= self.election_timeout {
            self.start_election()
        } else {
            Vec::new()
        }
    }

    fn start_election(&mut self) -> Vec<(NodeId, Message)> {
        self.role = Role::Candidate;
        self.current_term += 1;
        self.voted_for = Some(self.id);
        self.votes_received = 1; // vote for self
        self.timeout_counter = 0;

        let mut msgs = Vec::new();
        for &peer in &self.peers {
            let req = VoteRequest {
                term: self.current_term,
                candidate_id: self.id,
            };
            msgs.push((peer, Message::RequestVote(req)));
        }
        msgs
    }

    /// Process an incoming message from `from`. Returns any outbound messages.
    pub fn handle_message(&mut self, from: NodeId, msg: Message) -> Vec<(NodeId, Message)> {
        match msg {
            Message::RequestVote(req) => self.handle_vote_request(from, req),
            Message::RequestVoteResponse(resp) => self.handle_vote_response(from, resp),
        }
    }

    fn handle_vote_request(&mut self, from: NodeId, req: VoteRequest) -> Vec<(NodeId, Message)> {
        let mut msgs = Vec::new();

        if req.term < self.current_term {
            // Reject stale term
            let resp = VoteResponse {
                term: self.current_term,
                vote_granted: false,
                responder_id: self.id,
            };
            msgs.push((from, Message::RequestVoteResponse(resp)));
            return msgs;
        }

        // If term is newer, step down
        if req.term > self.current_term {
            self.current_term = req.term;
            self.become_follower();
        }

        let can_vote = self.voted_for.is_none() || self.voted_for == Some(req.candidate_id);
        let resp = VoteResponse {
            term: self.current_term,
            vote_granted: can_vote,
            responder_id: self.id,
        };
        if can_vote {
            self.voted_for = Some(req.candidate_id);
        }
        msgs.push((from, Message::RequestVoteResponse(resp)));
        msgs
    }

    fn handle_vote_response(&mut self, _from: NodeId, resp: VoteResponse) -> Vec<(NodeId, Message)> {
        let mut msgs = Vec::new();

        if resp.term > self.current_term {
            self.current_term = resp.term;
            self.become_follower();
            return msgs;
        }

        if self.role != Role::Candidate {
            return msgs;
        }

        if resp.vote_granted {
            self.votes_received += 1;
            let majority = (self.peers.len() + 1) / 2 + 1; // +1 includes self
            if self.votes_received >= majority {
                self.become_leader();
                // In a full Raft implementation we would now send AppendEntries.
            }
        }
        msgs
    }

    fn become_follower(&mut self) {
        self.role = Role::Follower;
        self.voted_for = None;
        self.votes_received = 0;
        self.timeout_counter = 0;
    }

    fn become_leader(&mut self) {
        self.role = Role::Leader;
        self.timeout_counter = 0;
    }
}