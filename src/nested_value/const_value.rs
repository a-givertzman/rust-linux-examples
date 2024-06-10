use std::fmt::Debug;
use regex::Regex;
use crate::nested_value::NestedValue;
///
/// Contains the constant value, returns on call get() method
pub struct ConstValue<T> {
    id: String,
    value: T,
}
//
//
impl<T> ConstValue<T> {
    ///
    /// Returns new instance of the [ConstValue]
    pub fn new(value: T) -> Self {
        let re = Regex::new(r"^(?:.*::)?(.+)$").unwrap();
        let raw_type_name = std::any::type_name::<Self>();
        let id = match re.captures(raw_type_name) {
            Some(caps) => caps.get(1).map_or(raw_type_name, |v| v.as_str()),
            None => raw_type_name,
        };
        Self {
            id: id.to_owned(),
            value,
        }
    }
}
//
//
impl<T: Clone> NestedValue<T> for ConstValue<T> {
    //
    //
    fn get(&self, _: &str) -> T {
        self.value.clone()
    }
}
//
//
impl<T: Debug> std::fmt::Debug for ConstValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstValue").field("id", &self.id).field("value", &self.value).finish()
    }
}