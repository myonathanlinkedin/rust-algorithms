pub const EPSILON: f64 = 1e-9;

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

    pub fn magnitude2(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    pub fn approx_eq(&self, other: &Self) -> bool {
        (self.re - other.re).abs() < EPSILON && (self.im - other.im).abs() < EPSILON
    }
}

// Implement basic arithmetic operators
use std::ops::{Add, Sub, Mul};

impl Add for Complex {
    type Output = Complex;
    fn add(self, rhs: Complex) -> Complex {
        Complex::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Sub for Complex {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Complex {
        Complex::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, rhs: Complex) -> Complex {
        Complex::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

// Helper: check if usize is power of two
pub fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

// Naïve DFT for verification (not exported)
pub fn dft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();
    let mut out = vec![Complex::zero(); n];
    let two_pi = std::f64::consts::TAU; // 2π
    for k in 0..n {
        let mut sum = Complex::zero();
        for (n_idx, x) in input.iter().enumerate() {
            let angle = -two_pi * (k as f64) * (n_idx as f64) / (n as f64);
            let twiddle = Complex::new(angle.cos(), angle.sin());
            sum = sum + (*x).clone() * twiddle;
        }
        out[k] = sum;
    }
    out
}

// In‑place Cooley‑Tukey Radix‑2 FFT (forward transform)
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

    // Iterative butterfly
    let mut len = 2usize;
    while len <= n {
        let half = len / 2;
        let theta = -std::f64::consts::TAU / (len as f64);
        let w_len = Complex::new(theta.cos(), theta.sin());
        for start in (0..n).step_by(len) {
            let mut w = Complex::one();
            for i in 0..half {
                let u = data[start + i].clone();
                let v = data[start + i + half].clone() * w.clone();
                data[start + i] = u.clone() + v.clone();
                data[start + i + half] = u - v;
                w = w * w_len.clone();
            }
        }
        len <<= 1;
    }
}

// Public verification function using DFT for testing
pub fn verify_fft(input: &[Complex]) -> bool {
    let mut fft_res = input.to_vec();
    fft(&mut fft_res);
    let dft_res = dft(input);
    fft_res
        .iter()
        .zip(dft_res.iter())
        .all(|(a, b)| a.approx_eq(b))
}