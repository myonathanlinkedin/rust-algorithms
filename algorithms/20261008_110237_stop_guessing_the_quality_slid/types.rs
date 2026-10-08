#[derive(Debug, Clone, PartialEq)]
pub struct QualitySearchResult {
    pub quality: u8,
    pub estimated_size: u64,
    pub success: bool,

}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchConfig {
    pub min_quality: u8,
    pub max_quality: u8,
    pub target_size: u64,
    pub tolerance: u64,

}

impl SearchConfig {
    pub fn new(min_quality: u8, max_quality: u8, target_size: u64, tolerance: u64) -> Self {
        SearchConfig {
            min_quality,
            max_quality,
            target_size,
            tolerance,
        }
    }
}