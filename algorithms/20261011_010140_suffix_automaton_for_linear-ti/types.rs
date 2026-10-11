#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub len: usize,
    pub link: Option<usize>,
    pub next: std::collections::HashMap<char, usize>,

}