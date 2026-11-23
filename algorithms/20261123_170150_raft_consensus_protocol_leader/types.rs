pub type NodeId = usize;
pub type Term = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Follower,
    Candidate,
    Leader,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VoteRequest {
    pub term: Term,
    pub candidate_id: NodeId,

}

#[derive(Debug, Clone, PartialEq)]
pub struct VoteResponse {
    pub term: Term,
    pub vote_granted: bool,
    pub responder_id: NodeId,

}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    RequestVote(VoteRequest),
    RequestVoteResponse(VoteResponse),
}