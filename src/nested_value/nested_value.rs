///
/// Interface for nested value
pub trait NestedValue<T> {
    ///
    /// Returns contained value.
    /// - First call get() method fetches the value.
    /// - Next time returns cached value.
    fn get(&self, key: &str) -> T;
}