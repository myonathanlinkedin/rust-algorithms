pub mod engine {
    use crate::types::types::{
        FeatureVector, FusionConfig, FusionError, FusedVector,
    };

    /// Fuse multiple modality feature vectors into a single vector.
    ///
    /// The algorithm performs a weighted element‑wise sum:
    /// `fused[i] = Σ (weight_j * modality_j[i])`
    /// and then normalizes by the sum of weights.
    ///
    /// # Errors
    ///
    /// Returns `FusionError` if:
    /// * No input vectors are supplied.
    /// * The number of weights does not match the number of vectors.
    /// * Vectors have differing lengths.
    /// * The sum of weights is zero (division by zero).
    pub fn fuse_features(
        config: &FusionConfig,
        inputs: &[FeatureVector],
    ) -> Result<FusedVector, FusionError> {
        if inputs.is_empty() {
            return Err(FusionError::EmptyInput);
        }

        if config.len() != inputs.len() {
            return Err(FusionError::WeightCountMismatch {
                expected: inputs.len(),
                found: config.len(),
            });
        }

        // Verify all vectors share the same length.
        let expected_len = inputs[0].len();
        for (idx, fv) in inputs.iter().enumerate() {
            if fv.len() != expected_len {
                return Err(FusionError::MismatchedLengths {
                    expected: expected_len,
                    found: fv.len(),
                });
            }
            // Silence unused variable warning in release builds.
            let _ = idx;
        }

        // Compute sum of weights.
        let weight_sum: f64 = config.weights.iter().copied().sum();
        if weight_sum.abs() < std::f64::EPSILON {
            return Err(FusionError::ZeroWeightSum);
        }

        // Allocate fused vector.
        let mut fused_data = vec![0.0_f64; expected_len];

        // Weighted accumulation.
        for (mod_idx, fv) in inputs.iter().enumerate() {
            let weight = config.weights[mod_idx];
            for i in 0..expected_len {
                fused_data[i] += weight * fv.data[i];
            }
        }

        // Normalization.
        for val in fused_data.iter_mut() {
            *val /= weight_sum;
        }

        Ok(FusedVector::new(fused_data))
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::types::types::{FeatureVector, FusionConfig};

        #[test]
        fn test_fuse_basic() {
            let fv1 = FeatureVector::new(vec![1.0, 2.0, 3.0]);
            let fv2 = FeatureVector::new(vec![4.0, 5.0, 6.0]);
            let config = FusionConfig::new(vec![0.5, 0.5]);
            let result = fuse_features(&config, &[fv1.clone(), fv2.clone()]).unwrap();
            let expected = vec![(0.5 * 1.0 + 0.5 * 4.0) / 1.0,
                                (0.5 * 2.0 + 0.5 * 5.0) / 1.0,
                                (0.5 * 3.0 + 0.5 * 6.0) / 1.0];
            assert_eq!(result.data, expected);
        }

        #[test]
        fn test_weighted_fusion() {
            let fv1 = FeatureVector::new(vec![1.0, 1.0]);
            let fv2 = FeatureVector::new(vec![3.0, 3.0]);
            let config = FusionConfig::new(vec![1.0, 3.0]); // heavier weight on fv2
            let result = fuse_features(&config, &[fv1, fv2]).unwrap();
            // (1*1 + 3*3) / 4 = (1 + 9)/4 = 2.5
            assert_eq!(result.data, vec![2.5, 2.5]);
        }

        #[test]
        fn test_mismatched_lengths_error() {
            let fv1 = FeatureVector::new(vec![1.0, 2.0]);
            let fv2 = FeatureVector::new(vec![3.0]); // shorter
            let config = FusionConfig::new(vec![0.5, 0.5]);
            let err = fuse_features(&config, &[fv1, fv2]).unwrap_err();
            match err {
                FusionError::MismatchedLengths { expected, found } => {
                    assert_eq!(expected, 2);
                    assert_eq!(found, 1);
                }
                _ => panic!("Unexpected error variant"),
            }
        }

        #[test]
        fn test_weight_count_mismatch_error() {
            let fv1 = FeatureVector::new(vec![1.0, 2.0]);
            let config = FusionConfig::new(vec![0.5, 0.5, 0.5]); // too many weights
            let err = fuse_features(&config, &[fv1]).unwrap_err();
            match err {
                FusionError::WeightCountMismatch { expected, found } => {
                    assert_eq!(expected, 1);
                    assert_eq!(found, 3);
                }
                _ => panic!("Unexpected error variant"),
            }
        }

        #[test]
        fn test_zero_weight_sum_error() {
            let fv1 = FeatureVector::new(vec![1.0, 2.0]);
            let config = FusionConfig::new(vec![0.0, 0.0]);
            let err = fuse_features(&config, &[fv1.clone(), fv1]).unwrap_err();
            match err {
                FusionError::ZeroWeightSum => {}
                _ => panic!("Unexpected error variant"),
            }
        }

        #[test]
        fn test_empty_input_error() {
            let config = FusionConfig::new(vec![]);
            let err = fuse_features(&config, &[]).unwrap_err();
            match err {
                FusionError::EmptyInput => {}
                _ => panic!("Unexpected error variant"),
            }
        }
    }
}