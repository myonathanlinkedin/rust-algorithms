mod core;
use core::{Complex, fft, verify_fft, EPSILON};

fn main() {
    // Simple sanity check: FFT of a constant sequence
    let mut data = vec![
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
        Complex::new(1.0, 0.0),
    ];
    fft(&mut data);
    // Expected: first element = sum, others = 0
    assert!((data[0].re - 4.0).abs() < EPSILON && data[0].im.abs() < EPSILON);
    for k in 1..data.len() {
        assert!(data[k].re.abs() < EPSILON && data[k].im.abs() < EPSILON);
    }

    // Verify against DFT for a random small vector
    let input = vec![
        Complex::new(0.0, 0.0),
        Complex::new(1.0, -1.0),
        Complex::new(0.5, 0.5),
        Complex::new(-0.3, 0.8),
        Complex::new(2.0, 2.0),
        Complex::new(-1.0, 1.0),
        Complex::new(0.0, -0.5),
        Complex::new(1.5, 0.0),
    ];
    assert!(verify_fft(&input));
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use core::{Complex, fft, is_power_of_two};

    #[test]
    fn test_is_power_of_two() {
        assert!(is_power_of_two(1));
        assert!(is_power_of_two(2));
        assert!(is_power_of_two(4));
        assert!(is_power_of_two(1024));
        assert!(!is_power_of_two(0));
        assert!(!is_power_of_two(3));
        assert!(!is_power_of_two(6));
    }

    #[test]
    fn test_fft_vs_dft_size_4() {
        let input = vec![
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(-1.0, 0.0),
        ];
        assert!(crate::core::verify_fft(&input));
    }

    #[test]
    fn test_fft_vs_dft_size_8_random() {
        let input = vec![
            Complex::new(0.7, -0.2),
            Complex::new(-1.3, 0.4),
            Complex::new(2.0, 2.0),
            Complex::new(-0.5, 1.1),
            Complex::new(0.0, 0.0),
            Complex::new(1.5, -0.7),
            Complex::new(-0.9, 0.3),
            Complex::new(0.2, -1.4),
        ];
        assert!(crate::core::verify_fft(&input));
    }

    #[test]
    #[should_panic(expected = "FFT length must be a power of two")]
    fn test_fft_non_power_of_two() {
        let mut data = vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(3.0, 0.0)];
        fft(&mut data);
    }
}