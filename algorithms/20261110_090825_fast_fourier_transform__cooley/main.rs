mod core;
use core::core::{Complex, fft, ifft};

fn main() {
    // Simple sanity check using the same data as the unit test.
    let mut data = vec![
        Complex::zero(),
        Complex::new(1.0, 0.0),
        Complex::zero(),
        Complex::zero(),
    ];
    fft(&mut data);
    // Expected result: [1, -i, -1, i]
    let expected = vec![
        Complex::new(1.0, 0.0),
        Complex::new(0.0, -1.0),
        Complex::new(-1.0, 0.0),
        Complex::new(0.0, 1.0),
    ];
    for (calc, exp) in data.iter().zip(expected.iter()) {
        assert!((calc.re - exp.re).abs() < 1e-12);
        assert!((calc.im - exp.im).abs() < 1e-12);
    }

    // Verify inverse restores original signal.
    ifft(&mut data);
    let restored = vec![
        Complex::zero(),
        Complex::new(1.0, 0.0),
        Complex::zero(),
        Complex::zero(),
    ];
    for (calc, exp) in data.iter().zip(restored.iter()) {
        assert!((calc.re - exp.re).abs() < 1e-12);
        assert!((calc.im - exp.im).abs() < 1e-12);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn round_trip_random_signal() {
        // A small deterministic "random" signal
        let mut signal = vec![
            Complex::new(0.3, -0.7),
            Complex::new(1.2, 0.4),
            Complex::new(-0.5, 0.9),
            Complex::new(0.0, -1.1),
        ];
        let original = signal.clone();
        fft(&mut signal);
        ifft(&mut signal);
        for (a, b) in signal.iter().zip(original.iter()) {
            assert!((a.re - b.re).abs() < 1e-12);
            assert!((a.im - b.im).abs() < 1e-12);
        }
    }
}