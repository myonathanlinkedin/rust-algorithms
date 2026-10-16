pub mod types {
    use std::fmt;

    #[derive(Debug, Clone, PartialEq)]
    pub struct FeatureVector {
        pub data: Vec<f64>,

    }

    impl FeatureVector {
        pub fn new(data: Vec<f64>) -> Self {
            FeatureVector { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FusionConfig {
        /// Weight for each modality. Must have the same length as the number of input vectors.
        pub weights: Vec<f64>,

    }

    impl FusionConfig {
        pub fn new(weights: Vec<f64>) -> Self {
            FusionConfig { weights }
        }

        pub fn len(&self) -> usize {
            self.weights.len()
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FusedVector {
        pub data: Vec<f64>,

    }

    impl FusedVector {
        pub fn new(data: Vec<f64>) -> Self {
            FusedVector { data }
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }
    }

    #[derive(Debug)]
    pub enum FusionError {
        EmptyInput,
        MismatchedLengths {
            expected: usize,
            found: usize,
        },
        WeightCountMismatch {
            expected: usize,
            found: usize,
        },
        ZeroWeightSum,
    }

    impl fmt::Display for FusionError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                FusionError::EmptyInput => write!(f, "No feature vectors provided"),
                FusionError::MismatchedLengths { expected, found } => write!(
                    f,
                    "Mismatched feature vector lengths: expected {}, found {}",
                    expected, found
                ),
                FusionError::WeightCountMismatch { expected, found } => write!(
                    f,
                    "Weight count mismatch: expected {}, found {}",
                    expected, found
                ),
                FusionError::ZeroWeightSum => write!(f, "Sum of weights is zero"),
            }
        }
    }

    impl std::error::Error for FusionError {}
}