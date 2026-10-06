pub mod types {
    #[derive(Debug, Clone, PartialEq)]
    pub struct ComputationResult {
        pub sum: i32,

    }

    // Simple async function that will be used in tests.
    pub async fn async_add(a: i32, b: i32) -> ComputationResult {
        // Simulate some async work with an immediate ready future.
        ComputationResult { sum: a + b }
    }

    // Another async function that performs multiple steps.
    pub async fn async_sequence(values: &[i32]) -> ComputationResult {
        let mut total = 0;
        for &v in values {
            // In a real async scenario we might await something here.
            total += v;
        }
        ComputationResult { sum: total }
    }
}