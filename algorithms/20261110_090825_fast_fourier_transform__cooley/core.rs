pub mod core {
    use std::ops::{Add, Sub, Mul, Neg};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Complex {
        pub re: f64,
        pub im: f64,

    }

    impl Complex {
        pub fn new(re: f64, im: f64) -> Self {
            Self { re, im }
        }

        pub fn zero() -> Self {
            Self::new(0.0, 0.0)
        }

        pub fn one() -> Self {
            Self::new(1.0, 0.0)
        }

        pub fn conj(&self) -> Self {
            Self::new(self.re, -self.im)
        }

        pub fn from_polar(r: f64, theta: f64) -> Self {
            Self::new(r * theta.cos(), r * theta.sin())
        }

        pub fn scale(&self, factor: f64) -> Self {
            Self::new(self.re * factor, self.im * factor)
        }
    }

    impl Add for Complex {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.re + rhs.re, self.im + rhs.im)
        }
    }

    impl Sub for Complex {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.re - rhs.re, self.im - rhs.im)
        }
    }

    impl Mul for Complex {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            Self::new(
                self.re * rhs.re - self.im * rhs.im,
                self.re * rhs.im + self.im * rhs.re,
            )
        }
    }

    impl Neg for Complex {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Self::new(-self.re, -self.im)
        }
    }

    #[inline]
    fn is_power_of_two(n: usize) -> bool {
        n != 0 && (n & (n - 1)) == 0
    }

    /// In‑place Cooley‑Tukey radix‑2 FFT.
    /// The length of `data` must be a power of two.
    pub fn fft(data: &mut [Complex]) {
        let n = data.len();
        assert!(is_power_of_two(n), "FFT length must be a power of two");

        // Bit‑reversal permutation
        let mut j = 0usize;
        for i in 1..n {
            let mut bit = n >> 1;
            while j & bit != 0 {
                j ^= bit;
                bit >>= 1;
            }
            j ^= bit;
            if i < j {
                data.swap(i, j);
            }
        }

        // Iterative Danielson‑Lanczos section
        let mut len = 2usize;
        while len <= n {
            let half = len / 2;
            let theta = -2.0 * std::f64::consts::PI / (len as f64);
            let wlen = Complex::from_polar(1.0, theta);
            for start in (0..n).step_by(len) {
                let mut w = Complex::one();
                for k in 0..half {
                    let u = data[start + k].clone();
                    let v = data[start + k + half].clone() * w.clone();
                    data[start + k] = u.clone() + v.clone();
                    data[start + k + half] = u - v;
                    w = w * wlen.clone();
                }
            }
            len <<= 1;
        }
    }

    /// In‑place inverse FFT.
    /// Performs conjugation, forward FFT, conjugation again, then scales by 1/n.
    pub fn ifft(data: &mut [Complex]) {
        let n = data.len();
        assert!(is_power_of_two(n), "IFFT length must be a power of two");

        // Conjugate
        for x in data.iter_mut() {
            *x = x.conj();
        }

        // Forward FFT
        fft(data);

        // Conjugate again and scale
        let scale = 1.0 / (n as f64);
        for x in data.iter_mut() {
            *x = x.conj().scale(scale);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn approx_eq(a: &Complex, b: &Complex, eps: f64) -> bool {
            (a.re - b.re).abs() < eps && (a.im - b.im).abs() < eps
        }

        #[test]
        fn test_fft_basic() {
            // Input: [0, 1, 0, 0]
            let mut data = vec![
                Complex::zero(),
                Complex::new(1.0, 0.0),
                Complex::zero(),
                Complex::zero(),
            ];
            fft(&mut data);
            let expected = vec![
                Complex::new(1.0, 0.0),
                Complex::new(0.0, -1.0),
                Complex::new(-1.0, 0.0),
                Complex::new(0.0, 1.0),
            ];
            for (a, b) in data.iter().zip(expected.iter()) {
                assert!(approx_eq(a, b, 1e-12));
            }
        }

        #[test]
        fn test_ifft_inverse() {
            let original = vec![
                Complex::new(0.5, -0.5),
                Complex::new(1.0, 0.0),
                Complex::new(-0.5, 0.5),
                Complex::new(0.0, -1.0),
            ];
            let mut transformed = original.clone();
            fft(&mut transformed);
            ifft(&mut transformed);
            for (a, b) in transformed.iter().zip(original.iter()) {
                assert!(approx_eq(a, b, 1e-12));
            }
        }

        #[test]
        fn test_fft_power_of_two_check() {
            let mut data = vec![Complex::zero(); 3]; // not a power of two
            let result = std::panic::catch_unwind(|| fft(&mut data));
            assert!(result.is_err());
        }
    }
}