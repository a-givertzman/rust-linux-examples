use std::fmt::Debug;
use regex::Regex;
use crate::nested_value::NestedValue;
///
/// Contains the mutable value, returns on call get() method
pub struct MutValue<T> {
    id: String,
    value: T,
}
//
//
impl<T> MutValue<T> {
    ///
    /// Returns new instance of the [MutValue]
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
impl<T: Clone> NestedValue<T> for MutValue<T> {
    //
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    //
    fn get(&self, _: &str) -> Result<T, String> {
        Ok(self.value.clone())
    }
    //
    //
    fn store(&mut self, _: &str, value: T) -> Result<(), String> {
        self.value = value;
        Ok(())
    }
}
//
//
impl<T: Debug> std::fmt::Debug for MutValue<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MutValue").field("id", &self.id).field("value", &self.value).finish()
    }
}