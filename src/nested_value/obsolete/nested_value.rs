///
/// Interface for nested value
pub trait NestedValue<T> {
    ///
    /// Returns the idinifier of the nested values node
    fn id(&self) -> String;
    ///
    /// Do not use this method, used for internal purposes
    fn init(&mut self, key: &str);
    ///
    /// Returns contained value by nested value path
    /// (path required for the MultiValue, in other cases can be empty).
    /// - First call get() method fetches the value.
    /// - Next time returns cached value.
    fn get(&self, key: &str) -> Result<T, String>;
    ///
    /// Stores a new value into the node of the nested values by it's path
    /// (path required for the MultiValue, in other cases can be empty).
    fn store(&mut self, editor: &str, key: &str, value: T) -> Result<(), String> {
        (_, _, _) = (editor, key, value);
        panic!("{}.store | Store does not supported", self.id())
    }
    ///
    /// Returns history of edited value by nested value path, actual for MutValue only
    fn edited(&self, key: &str) -> Result<Vec<String>, String> {
        _ = key;
        Ok(vec![])
    }
}
