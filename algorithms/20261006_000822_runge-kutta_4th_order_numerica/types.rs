pub type State = Vec<f64>;

/// Trait representing a system of first‑order ordinary differential equations:
///     dy/dt = f(t, y)
/// where `y` is a state vector of dimension `dim()`.
pub trait ODE {
    /// Returns the dimension of the state vector.
    fn dim(&self) -> usize;

    /// Computes the derivative vector at time `t` for state `y`.
    ///
    /// # Arguments
    ///
    /// * `t` – Current time.
    /// * `y` – Slice containing the current state (length must equal `dim()`).
    ///
    /// # Returns
    ///
    /// A `Vec<f64>` of length `dim()` containing dy/dt.
    fn derivative(&self, t: f64, y: &[f64]) -> Vec<f64>;
}

/// Simple wrapper that stores a boxed closure implementing the ODE.
///
/// This allows users to create an ODE from any compatible closure without
/// defining a concrete type.
pub struct ODEFn {
    dim: usize,
    f: Box<dyn Fn(f64, &[f64]) -> Vec<f64>>,
}

impl ODEFn {
    /// Constructs a new `ODEFn`.
    ///
    /// # Arguments
    ///
    /// * `dim` – Dimension of the state vector.
    /// * `f` – Closure that computes dy/dt.
    pub fn new<F>(dim: usize, f: F) -> Self
    where
        F: 'static + Fn(f64, &[f64]) -> Vec<f64>,
    {
        ODEFn {
            dim,
            f: Box::new(f),
        }
    }
}

impl ODE for ODEFn {
    fn dim(&self) -> usize {
        self.dim
    }

    fn derivative(&self, t: f64, y: &[f64]) -> Vec<f64> {
        (self.f)(t, y)
    }
}
