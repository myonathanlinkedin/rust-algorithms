mod core;
use core::RK4Integrator;

/// Simple absolute tolerance comparison for floating point vectors.
fn approx_eq(a: &Vec<f64>, b: &Vec<f64>, tol: f64) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for i in 0..a.len() {
        if (a[i] - b[i]).abs() > tol {
            return false;
        }
    }
    true
}

fn main() {
    // Verify exponential growth dy/dt = y, y(0)=1, exact y(t)=e^t.
    let f = |_: f64, y: &Vec<f64>| -> Vec<f64> { vec![y[0]] };
    let y0 = vec![1.0];
    let h = 0.1;
    let steps = 10; // integrate to t = 1.0
    let (_times, states) = RK4Integrator::integrate(&f, 0.0, &y0, h, steps);
    let y_final = &states[states.len() - 1];
    let exact = vec![(std::f64::consts::E)];
    assert!(approx_eq(y_final, &exact, 1e-6));
}

// === TESTS ===
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exponential_growth() {
        // dy/dt = y, y(0) = 1 => y(t) = e^t
        let f = |_: f64, y: &Vec<f64>| -> Vec<f64> { vec![y[0]] };
        let y0 = vec![1.0];
        let h = 0.05;
        let steps = 40; // t = 2.0
        let (_times, states) = RK4Integrator::integrate(&f, 0.0, &y0, h, steps);
        let y_final = &states[states.len() - 1];
        let exact = vec![(2.0_f64).exp()];
        assert!(approx_eq(y_final, &exact, 1e-7));
    }

    #[test]
    fn test_exponential_decay() {
        // dy/dt = -y, y(0) = 2 => y(t) = 2*e^{-t}
        let f = |_: f64, y: &Vec<f64>| -> Vec<f64> { vec![-y[0]] };
        let y0 = vec![2.0];
        let h = 0.1;
        let steps = 30; // t = 3.0
        let (_times, states) = RK4Integrator::integrate(&f, 0.0, &y0, h, steps);
        let y_final = &states[states.len() - 1];
        let exact = vec![2.0 * (-3.0_f64).exp()];
        assert!(approx_eq(y_final, &exact, 1e-6));
    }

    #[test]
    fn test_harmonic_oscillator() {
        // System: x' = v, v' = -x  (simple harmonic oscillator)
        // Exact solution with x(0)=1, v(0)=0: x(t)=cos(t), v(t)=-sin(t)
        let f = |_: f64, y: &Vec<f64>| -> Vec<f64> {
            // y[0] = x, y[1] = v
            vec![y[1], -y[0]]
        };
        let y0 = vec![1.0, 0.0];
        let h = 0.01;
        let steps = 628; // t ≈ 2π
        let (_times, states) = RK4Integrator::integrate(&f, 0.0, &y0, h, steps);
        let y_final = &states[states.len() - 1];
        let exact = vec![(2.0_f64 * std::f64::consts::PI).cos(), -(2.0_f64 * std::f64::consts::PI).sin()];
        assert!(approx_eq(y_final, &exact, 1e-5));
    }

    #[test]
    #[should_panic(expected = "step size must be positive")]
    fn test_invalid_step_size() {
        let f = |_: f64, y: &Vec<f64>| -> Vec<f64> { vec![y[0]] };
        let y0 = vec![1.0];
        // This should panic because h <= 0
        RK4Integrator::step(&f, 0.0, &y0, 0.0);
    }
}