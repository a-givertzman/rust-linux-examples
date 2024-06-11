use std::fmt::Debug;
use crate::nested_value::NestedValue;
///
/// Contains the constant value, returns on call get() method
pub struct ConstValue<T> {
    id: String,
    inited: bool,
    value: T,
}
//
//
impl<T> ConstValue<T> {
    ///
    /// Returns new instance of the [ConstValue]
    pub fn new(value: T) -> Self {
        Self {
            id: "ConstValue".to_owned(),
            inited: false,
            value,
        }
    }
}
//
//
impl<T: Clone> NestedValue<T> for ConstValue<T> {
    //
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    //
    fn init(&mut self, key: &str) {
        self.id = key.to_owned();
        self.inited = true;
    }
    //
    //
    fn get(&self, _: &str) -> Result<T, String> {
        Ok(self.value.clone())
    }
    //
    //
    fn store(&mut self, editor: &str, _: &str, _: T) -> Result<(), String> {
        Err(format!("{}.store | Store does not supported for constant, requested from '{}'", self.id, editor))
    }
}
//
//
impl<T: Debug> std::fmt::Debug for ConstValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstValue").field("id", &self.id).field("value", &self.value).finish()
    }
}