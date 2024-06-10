use log::warn;
///
/// Interface for nested value
pub trait NestedValue<T> {
    ///
    /// Returns the idinifier of the nested values node
    fn id(&self) -> String;
    ///
    /// Returns contained value by nested value path.
    /// - First call get() method fetches the value.
    /// - Next time returns cached value.
    fn get(&self, key: &str) -> T;
    ///
    /// Stores a new value into the node of the nested values by it's path.
    fn store(&mut self, key: &str, value: T) {
        (_, _) = (key, value);
        warn!("{}.store | Store does not supported", self.id())
    }
}