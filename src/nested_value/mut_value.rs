use std::fmt::Debug;
use chrono::Utc;
use crate::nested_value::NestedValue;
///
/// Contains the mutable value, returns on call get() method
pub struct MutValue<T> {
    id: String,
    inited: bool,
    value: T,
    edited: Vec<String>,
}
//
//
impl<T> MutValue<T> {
    ///
    /// Returns new instance of the [MutValue]
    pub fn new(value: T) -> Self {
        Self {
            id: std::any::type_name::<Self>().to_owned(),
            inited: false,
            value,
            edited: vec![],
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
    fn init(&mut self, parent: &str) {
        self.id = parent.to_owned();
        self.inited = true;
    }
    //
    //
    fn get(&self, _: &str) -> Result<T, String> {
        Ok(self.value.clone())
    }
    //
    //
    fn store(&mut self, editor: &str, _: &str, value: T) -> Result<(), String> {
        self.value = value;
        self.edited.push(format!("{}. {} - {}", self.edited.len() + 1, Utc::now(), editor));
        println!("{}.store | edited: {:#?}", self.id, self.edited);
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