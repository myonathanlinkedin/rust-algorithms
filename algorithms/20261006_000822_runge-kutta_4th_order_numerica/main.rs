mod types;
mod engine;

use crate::types::{ODEFn, State};
use crate::engine::{integrate, rk4_step};

/// Helper to compare two floating‑point numbers within a tolerance.
fn approx_eq(a: f64, b: f64, eps: f64) -> bool {
    (a - b).abs() <= eps
}

/// Simple harmonic oscillator:
///   y₁' = y₂
///   y₂' = -y₁
///
/// Exact solution for initial condition (y₁(0), y₂(0)) = (1, 0):
///   y₁(t) = cos(t)
///   y₂(t) = -sin(t)
fn harmonic_oscillator() -> ODEFn {
    ODEFn::new(2, |_, y| vec![y[1], -y[0]])
}

fn main() {
    // Parameters for the test
    let h = 0.01_f64;               // step size
    let steps = (2.0 * std::f64::consts::PI / h).round() as usize; // one full period
    let t0 = 0.0_f64;
    let y0 = vec![1.0_f64, 0.0_f64]; // (cos(0), -sin(0))

    let ode = harmonic_oscillator();

    // Perform integration
    let trajectory = integrate(&ode, t0, &y0, h, steps);

    // Verify final state is close to the initial state (periodicity)
    let final_state = trajectory.last().expect("trajectory is non‑empty");
    let eps = 1e-4;
    assert!(
        approx_eq(final_state[0], y0[0], eps) && approx_eq(final_state[1], y0[1], eps),
        "Final state deviates from expected periodic solution: got {:?}, expected {:?}",
        final_state,
        y0
    );

    // Additional spot checks against analytical solution at t = π/2
    let t_half_pi = std::f64::consts::FRAC_PI_2;
    let idx = (t_half_pi / h).round() as usize;
    let state_half_pi = &trajectory[idx];
    let expected = vec![0.0_f64, -1.0_f64]; // cos(π/2)=0, -sin(π/2)=-1
    let eps_spot = 5e-4;
    assert!(
        approx_eq(state_half_pi[0], expected[0], eps_spot)
            && approx_eq(state_half_pi[1], expected[1], eps_spot),
        "State at t=π/2 deviates: got {:?}, expected {:?}",
        state_half_pi,
        expected
    );

    // Edge case: zero step size should return the initial state unchanged
    let zero_step = rk4_step(&ode, t0, &y0, 0.0);
    assert!(
        approx_eq(zero_step[0], y0[0], 1e-12) && approx_eq(zero_step[1], y0[1], 1e-12),
        "Zero step size altered the state"
    );

    // Edge case: zero integration steps returns only the initial state
    let zero_steps_traj = integrate(&ode, t0, &y0, h, 0);
    assert_eq!(zero_steps_traj.len(), 1, "Zero steps should yield single state");
    assert!(
        approx_eq(zero_steps_traj[0][0], y0[0], 1e-12)
            && approx_eq(zero_steps_traj[0][1], y0[1], 1e-12),
        "Zero steps trajectory does not match initial state"
    );

    // Demo output (optional, not required for tests)
    println!("Integration completed successfully. Final state: {:?}", final_state);
}
