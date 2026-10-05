/// A module modeling a simple pendulum clock.
///
/// The pendulum's period is given by the classic formula:
/// `T = 2π * sqrt(L / g)`, where `L` is the pendulum length (meters)
/// and `g` is the local gravitational acceleration (m/s²).
///
/// This implementation provides:
/// * Validation of physical parameters.
/// * Computation of period, frequency, and number of oscillations over a duration.
/// * A production‑ready error type with `std::fmt::Display` and `std::error::Error`.
/// * A `main` function containing self‑checking assertions.

use std::error::Error;
use std::fmt;

/// Errors that can arise when constructing or using a `Pendulum`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PendulumError {
    /// The pendulum length must be a positive, finite number.
    InvalidLength,
    /// The gravitational acceleration must be a positive, finite number.
    InvalidGravity,
}

impl fmt::Display for PendulumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PendulumError::InvalidLength => write!(f, "pendulum length must be > 0 and finite"),
            PendulumError::InvalidGravity => write!(f, "gravity must be > 0 and finite"),
        }
    }
}

impl Error for PendulumError {}

/// Represents a simple pendulum used in a clock.
#[derive(Debug, Clone, Copy)]
pub struct Pendulum {
    /// Length of the pendulum in meters.
    length_m: f64,
    /// Gravitational acceleration in meters per second squared.
    gravity_m_s2: f64,
}

impl Pendulum {
    /// Creates a new `Pendulum`.
    ///
    /// # Errors
    ///
    /// Returns `PendulumError::InvalidLength` if `length_m` is not positive or is NaN/Infinite.
    /// Returns `PendulumError::InvalidGravity` if `gravity_m_s2` is not positive or is NaN/Infinite.
    pub fn new(length_m: f64, gravity_m_s2: f64) -> Result<Self, PendulumError> {
        if !(length_m > 0.0) || !length_m.is_finite() {
            return Err(PendulumError::InvalidLength);
        }
        if !(gravity_m_s2 > 0.0) || !gravity_m_s2.is_finite() {
            return Err(PendulumError::InvalidGravity);
        }
        Ok(Pendulum {
            length_m,
            gravity_m_s2,
        })
    }

    /// Returns the pendulum length in meters.
    pub fn length(&self) -> f64 {
        self.length_m
    }

    /// Returns the gravitational acceleration in m/s².
    pub fn gravity(&self) -> f64 {
        self.gravity_m_s2
    }

    /// Computes the period `T` of a single swing (full back‑and‑forth) in seconds.
    ///
    /// Formula: `T = 2π * sqrt(L / g)`.
    pub fn period(&self) -> f64 {
        const TWO_PI: f64 = std::f64::consts::TAU; // TAU = 2π
        TWO_PI * (self.length_m / self.gravity_m_s2).sqrt()
    }

    /// Computes the frequency `f` in Hertz (oscillations per second).
    pub fn frequency(&self) -> f64 {
        1.0 / self.period()
    }

    /// Returns the number of complete oscillations that occur over `duration_s` seconds.
    ///
    /// The result is a floating point value; callers may round as needed.
    pub fn oscillations(&self, duration_s: f64) -> f64 {
        if duration_s <= 0.0 || !duration_s.is_finite() {
            return 0.0;
        }
        duration_s / self.period()
    }

    /// Returns the number of complete oscillations (rounded down) that occur over `duration_s` seconds.
    pub fn whole_oscillations(&self, duration_s: f64) -> u64 {
        self.oscillations(duration_s).floor() as u64
    }
}

/// Helper function for approximate equality of floating point numbers.
///
/// Returns `true` if `a` and `b` differ by less than `epsilon`.
fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

fn main() {
    // Typical pendulum length for a 30‑minute clock (approx. 0.5 m).
    let length = 0.5_f64;
    let gravity = 9.80665_f64; // Standard gravity.

    // Construct the pendulum; unwrap is safe because parameters are valid.
    let pendulum = Pendulum::new(length, gravity).expect("valid pendulum parameters");

    // Expected period using the analytical formula.
    // T = 2π * sqrt(L / g) ≈ 1.419 s for L = 0.5 m.
    let expected_period = 2.0 * std::f64::consts::PI * (length / gravity).sqrt();
    let computed_period = pendulum.period();

    // Verify period within a tight tolerance (1 µs).
    assert!(
        approx_eq(computed_period, expected_period, 1e-9),
        "Period mismatch: computed {}, expected {}",
        computed_period,
        expected_period
    );

    // Verify frequency is the reciprocal of period.
    let expected_frequency = 1.0 / expected_period;
    let computed_frequency = pendulum.frequency();
    assert!(
        approx_eq(computed_frequency, expected_frequency, 1e-12),
        "Frequency mismatch: computed {}, expected {}",
        computed_frequency,
        expected_frequency
    );

    // Simulate a 30‑minute interval (1800 s).
    let duration_seconds = 30.0 * 60.0;
    let total_oscillations = pendulum.oscillations(duration_seconds);
    let whole_oscillations = pendulum.whole_oscillations(duration_seconds);

    // Expected number of oscillations = duration / period.
    let expected_osc = duration_seconds / expected_period;
    assert!(
        approx_eq(total_oscillations, expected_osc, 1e-9),
        "Oscillation count mismatch: computed {}, expected {}",
        total_oscillations,
        expected_osc
    );

    // The whole number of oscillations should be the floor of the exact count.
    assert_eq!(whole_oscillations, expected_osc.floor() as u64);

    // Edge‑case tests.
    // Zero or negative duration yields zero oscillations.
    assert_eq!(pendulum.whole_oscillations(0.0), 0);
    assert_eq!(pendulum.whole_oscillations(-10.0), 0);

    // Invalid construction attempts.
    assert_eq!(
        Pendulum::new(0.0, gravity).unwrap_err(),
        PendulumError::InvalidLength
    );
    assert_eq!(
        Pendulum::new(length, -9.8).unwrap_err(),
        PendulumError::InvalidGravity
    );

    // All assertions passed.
    println!("All pendulum tests passed.");
}