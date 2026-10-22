use std::sync::Arc;

/// A copy‑on‑write vector. Internally uses `Arc<Vec<T>>` to share data
/// between clones. Mutating operations trigger a clone of the underlying
/// buffer when the reference count is greater than one.
#[derive(Debug, Clone, PartialEq)]
pub struct CowVec<T>
where
    T: Clone,
{
    data: Arc<Vec<T>>,
}

impl<T> CowVec<T>
where
    T: Clone,
{
    /// Creates an empty `CowVec`.
    pub fn new() -> Self {
        CowVec {
            data: Arc::new(Vec::new()),
        }
    }

    /// Constructs a `CowVec` from a slice by cloning its contents.
    pub fn from_slice(slice: &[T]) -> Self {
        CowVec {
            data: Arc::new(slice.to_vec()),
        }
    }

    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Returns a reference to the element at `index`, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    /// Returns a mutable reference to the element at `index`.
    ///
    /// This method performs copy‑on‑write: if the underlying buffer is shared,
    /// it is cloned before the mutable reference is handed out.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let vec = Arc::make_mut(&mut self.data);
        vec.get_mut(index)
    }

    /// Sets the element at `index` to `value`. Panics if `index` is out of bounds.
    pub fn set(&mut self, index: usize, value: T) {
        let vec = Arc::make_mut(&mut self.data);
        if index >= vec.len() {
            panic!("index out of bounds");
        }
        vec[index] = value;
    }

    /// Appends `value` to the end of the vector.
    pub fn push(&mut self, value: T) {
        let vec = Arc::make_mut(&mut self.data);
        vec.push(value);
    }

    /// Returns a slice of the whole buffer.
    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    /// Returns the current strong reference count. Useful for testing.
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }
}