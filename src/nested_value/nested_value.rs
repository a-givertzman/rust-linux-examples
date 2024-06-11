///
/// Interface for nested value
pub trait NestedValue<T> {
    ///
    /// Returns the idinifier of the nested values node
    fn id(&self) -> String;
    ///
    /// 
    fn init(&mut self, parent: &str);
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
}
