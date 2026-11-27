use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub children: [Option<usize>; 26],
    pub fail: usize,
    pub output: Vec<usize>,

}

#[derive(Debug, Clone, PartialEq)]
pub struct AhoCorasick {
    pub patterns: Vec<String>,
    pub nodes: Vec<Node>,

}