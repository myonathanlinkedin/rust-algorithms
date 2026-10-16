mod types;
mod engine;

use types::types::{FeatureVector, FusionConfig};
use engine::engine::fuse_features;

fn main() {
    // Demo: simple three‑modality fusion.
    let modality_a = FeatureVector::new(vec![0.2, 0.4, 0.6, 0.8]);
    let modality_b = FeatureVector::new(vec![1.0, 0.9, 0.8, 0.7]);
    let modality_c = FeatureVector::new(vec![0.5, 0.5, 0.5, 0.5]);

    let config = FusionConfig::new(vec![0.3, 0.5, 0.2]); // weights sum to 1.0

    let fused = fuse_features(&config, &[modality_a, modality_b, modality_c])
        .expect("Fusion should succeed");

    // Expected manual calculation:
    // fused[i] = (0.3*a[i] + 0.5*b[i] + 0.2*c[i]) / (0.3+0.5+0.2) = same as weighted sum because sum=1.
    let expected = vec![
        0.3 * 0.2 + 0.5 * 1.0 + 0.2 * 0.5,
        0.3 * 0.4 + 0.5 * 0.9 + 0.2 * 0.5,
        0.3 * 0.6 + 0.5 * 0.8 + 0.2 * 0.5,
        0.3 * 0.8 + 0.5 * 0.7 + 0.2 * 0.5,
    ];

    // Verify each component within a tiny epsilon due to floating‑point arithmetic.
    for (computed, &exp) in fused.data.iter().zip(expected.iter()) {
        assert!((computed - exp).abs() < 1e-12);
    }

    // Additional edge‑case assertions (mirroring unit tests but in main).

    // 1. Empty input should error.
    let empty_cfg = FusionConfig::new(vec![]);
    assert!(engine::engine::fuse_features(&empty_cfg, &[]).is_err());

    // 2. Mismatched vector lengths.
    let short_vec = FeatureVector::new(vec![1.0, 2.0]);
    let long_vec = FeatureVector::new(vec![3.0, 4.0, 5.0]);
    let cfg = FusionConfig::new(vec![0.5, 0.5]);
    let err = engine::engine::fuse_features(&cfg, &[short_vec, long_vec]).unwrap_err();
    match err {
        crate::types::types::FusionError::MismatchedLengths { expected, found } => {
            assert_eq!(expected, 2);
            assert_eq!(found, 3);
        }
        _ => panic!("Unexpected error variant for mismatched lengths"),
    }

    // 3. Weight count mismatch.
    let cfg = FusionConfig::new(vec![0.7]); // only one weight for two inputs
    let fv1 = FeatureVector::new(vec![1.0, 1.0]);
    let fv2 = FeatureVector::new(vec![2.0, 2.0]);
    let err = engine::engine::fuse_features(&cfg, &[fv1, fv2]).unwrap_err();
    match err {
        crate::types::types::FusionError::WeightCountMismatch { expected, found } => {
            assert_eq!(expected, 2);
            assert_eq!(found, 1);
        }
        _ => panic!("Unexpected error variant for weight count mismatch"),
    }

    // 4. Zero weight sum.
    let cfg = FusionConfig::new(vec![0.0, 0.0]);
    let fv1 = FeatureVector::new(vec![1.0, 2.0]);
    let fv2 = FeatureVector::new(vec![3.0, 4.0]);
    let err = engine::engine::fuse_features(&cfg, &[fv1, fv2]).unwrap_err();
    match err {
        crate::types::types::FusionError::ZeroWeightSum => {}
        _ => panic!("Unexpected error variant for zero weight sum"),
    }

    // All assertions passed.
}