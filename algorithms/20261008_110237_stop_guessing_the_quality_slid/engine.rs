use crate::types::{QualitySearchResult, SearchConfig};

/// Simulates JPEG file size based on quality level.
/// In a real implementation, this would encode the image and return the actual byte size.
/// Here we use a deterministic model: size increases non-linearly with quality.
/// Base size at quality 0 is 1000 bytes, and each quality point adds increasing bytes.
pub fn simulate_jpeg_size(quality: u8) -> u64 {
    // Model: size = 1000 + (quality as u64) * (quality as u64) * 10 + (quality as u64) * 50
    // This creates a convex, monotonically increasing function
    let q = quality as u64;
    1000 + q * q * 10 + q * 50
}

/// Binary search for the JPEG quality level that produces a file size closest to target.
/// Returns the quality level and whether the size was within tolerance.
pub fn binary_search_quality(config: &SearchConfig) -> QualitySearchResult {
    let mut low = config.min_quality;
    let mut high = config.max_quality;
    let mut best_quality = config.min_quality;
    let mut best_size = simulate_jpeg_size(config.min_quality);
    let mut best_diff = best_size.abs_diff(config.target_size);

    while low <= high {
        let mid = low + (high - low) / 2;
        let size = simulate_jpeg_size(mid);
        let diff = size.abs_diff(config.target_size);

        if diff < best_diff {
            best_diff = diff;
            best_quality = mid;
            best_size = size;
        }

        if size < config.target_size {
            low = mid + 1;
        } else if size > config.target_size {
            high = mid.saturating_sub(1);
        } else {
            // Exact match
            return QualitySearchResult {
                quality: mid,
                estimated_size: size,
                success: true,
            };
        }
    }

    let success = best_diff <= config.tolerance;
    QualitySearchResult {
        quality: best_quality,
        estimated_size: best_size,
        success,
    }
}

/// Find the minimum quality that produces a file size >= target_size.
pub fn find_min_quality_for_size(target_size: u64, min_q: u8, max_q: u8) -> Option<u8> {
    let mut low = min_q;
    let mut high = max_q;
    let mut result: Option<u8> = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        let size = simulate_jpeg_size(mid);
        if size >= target_size {
            result = Some(mid);
            high = mid.saturating_sub(1);
        } else {
            low = mid + 1;
        }
    }

    result
}

/// Find the maximum quality that produces a file size <= target_size.
pub fn find_max_quality_for_size(target_size: u64, min_q: u8, max_q: u8) -> Option<u8> {
    let mut low = min_q;
    let mut high = max_q;
    let mut result: Option<u8> = None;

    while low <= high {
        let mid = low + (high - low) / 2;
        let size = simulate_jpeg_size(mid);
        if size <= target_size {
            result = Some(mid);
            low = mid + 1;
        } else {
            high = mid.saturating_sub(1);
        }
    }

    result
}