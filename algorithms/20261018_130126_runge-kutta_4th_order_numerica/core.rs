use std::ops::Add;

/// Represents the Runge‑Kutta 4th order integrator for systems of ODEs.
///
/// The state vector `y` is a `Vec<f64>`; the derivative function `f`
/// must have the signature `Fn(f64, &Vec<f64>) -> Vec<f64>`.
pub struct RK4Integrator;

impl RK4Integrator {
    /// Perform a single RK4 step.
    ///
    /// * `f` – derivative function `dy/dt = f(t, y)`.
    /// * `t` – current time.
    /// * `y` – current state vector.
    /// * `h` – step size (must be > 0).
    ///
    /// Returns the state vector at `t + h`.
    pub fn step<F>(f: &F, t: f64, y: &Vec<f64>, h: f64) -> Vec<f64>
    where
        F: Fn(f64, &Vec<f64>) -> Vec<f64>,
    {
        assert!(h > 0.0, "step size must be positive");
        let k1 = f(t, y);
        let y_k2 = RK4Integrator::vec_add_scaled(y, &k1, h * 0.5);
        let k2 = f(t + h * 0.5, &y_k2);
        let y_k3 = RK4Integrator::vec_add_scaled(y, &k2, h * 0.5);
        let k3 = f(t + h * 0.5, &y_k3);
        let y_k4 = RK4Integrator::vec_add_scaled(y, &k3, h);
        let k4 = f(t + h, &y_k4);

        // y_{n+1} = y_n + h/6 * (k1 + 2*k2 + 2*k3 + k4)
        let mut result = y.clone();
        for i in 0..result.len() {
            result[i] += h
                / 6.0
                * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        }
        result
    }

    /// Integrate from `t0` to `t0 + steps * h`.
    ///
    /// Returns a tuple `(times, states)` where `times[i]` corresponds to
    /// `states[i]`.
    pub fn integrate<F>(
        f: &F,
        t0: f64,
        y0: &Vec<f64>,
        h: f64,
        steps: usize,
    ) -> (Vec<f64>, Vec<Vec<f64>>)
    where
        F: Fn(f64, &Vec<f64>) -> Vec<f64>,
    {
        let mut times = Vec::with_capacity(steps + 1);
        let mut states = Vec::with_capacity(steps + 1);
        times.push(t0);
        states.push(y0.clone());

        let mut t = t0;
        let mut y = y0.clone();

        for _ in 0..steps {
            y = RK4Integrator::step(f, t, &y, h);
            t += h;
            times.push(t);
            states.push(y.clone());
        }

        (times, states)
    }

    /// Helper: `y + scale * v` (element‑wise).
    fn vec_add_scaled(y: &Vec<f64>, v: &Vec<f64>, scale: f64) -> Vec<f64> {
        assert_eq!(y.len(), v.len(), "vector size mismatch");
        let mut result = Vec::with_capacity(y.len());
        for i in 0..y.len() {
            result.push(y[i] + scale * v[i]);
        }
        result
    }
}