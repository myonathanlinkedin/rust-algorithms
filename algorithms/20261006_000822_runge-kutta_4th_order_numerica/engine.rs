use crate::types::{ODE, State};

/// Performs a single Runge–Kutta 4th order step.
///
/// # Arguments
///
/// * `ode` – Reference to an object implementing `ODE`.
/// * `t` – Current time.
/// * `y` – Current state slice (length must equal `ode.dim()`).
/// * `h` – Step size (must be non‑negative).
///
/// # Returns
///
/// A new `Vec<f64>` containing the state at `t + h`.
///
/// # Panics
///
/// Panics if `y.len()` does not match `ode.dim()` or if `h` is negative.
pub fn rk4_step<O: ODE>(ode: &O, t: f64, y: &[f64], h: f64) -> State {
    assert_eq!(y.len(), ode.dim(), "state length mismatch");
    assert!(h >= 0.0, "step size must be non‑negative");

    if h == 0.0 {
        return y.to_vec();
    }

    let dim = ode.dim();

    // k1 = f(t, y)
    let k1 = ode.derivative(t, y);

    // y + h/2 * k1
    let y_mid1: Vec<f64> = y
        .iter()
        .zip(k1.iter())
        .map(|(&yi, &k1i)| yi + 0.5 * h * k1i)
        .collect();

    // k2 = f(t + h/2, y + h/2 * k1)
    let k2 = ode.derivative(t + 0.5 * h, &y_mid1);

    // y + h/2 * k2
    let y_mid2: Vec<f64> = y
        .iter()
        .zip(k2.iter())
        .map(|(&yi, &k2i)| yi + 0.5 * h * k2i)
        .collect();

    // k3 = f(t + h/2, y + h/2 * k2)
    let k3 = ode.derivative(t + 0.5 * h, &y_mid2);

    // y + h * k3
    let y_end: Vec<f64> = y
        .iter()
        .zip(k3.iter())
        .map(|(&yi, &k3i)| yi + h * k3i)
        .collect();

    // k4 = f(t + h, y + h * k3)
    let k4 = ode.derivative(t + h, &y_end);

    // Combine to produce y_{n+1}
    y.iter()
        .zip(k1.iter())
        .zip(k2.iter())
        .zip(k3.iter())
        .zip(k4.iter())
        .map(|(((((&yi, &k1i), &k2i), &k3i), &k4i))| {
            yi + (h / 6.0) * (k1i + 2.0 * k2i + 2.0 * k3i + k4i)
        })
        .collect()
}

/// Integrates an ODE from `t0` forward for `steps` steps of size `h`.
///
/// Returns a vector containing the state at each step, including the initial state.
///
/// # Arguments
///
/// * `ode` – ODE system to integrate.
/// * `t0` – Initial time.
/// * `y0` – Initial state slice.
/// * `h` – Step size (must be non‑negative).
/// * `steps` – Number of integration steps (may be zero).
///
/// # Panics
///
/// Panics if `y0.len()` does not match `ode.dim()` or if `h` is negative.
pub fn integrate<O: ODE>(ode: &O, t0: f64, y0: &[f64], h: f64, steps: usize) -> Vec<State> {
    assert_eq!(y0.len(), ode.dim(), "initial state length mismatch");
    assert!(h >= 0.0, "step size must be non‑negative");

    let mut trajectory = Vec::with_capacity(steps + 1);
    trajectory.push(y0.to_vec());

    let mut t = t0;
    let mut y = y0.to_vec();

    for _ in 0..steps {
        y = rk4_step(ode, t, &y, h);
        t += h;
        trajectory.push(y.clone());
    }

    trajectory
}
