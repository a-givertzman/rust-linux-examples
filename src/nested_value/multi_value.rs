use indexmap::IndexMap;
use regex::Regex;
use crate::nested_value::NestedValue;
///
/// Containing multiple nested values
pub struct MultiValue<T> {
    id: String,
    values: IndexMap<String, Box<dyn NestedValue<T>>>
}
//
//
impl<T> MultiValue<T> {
    ///
    /// Returns new instance of the [MultiValue]
    /// - values - array of the pairs 'key' - 'NestedValue' 
    pub fn new<const N: usize>(values: [(&str, Box<dyn NestedValue<T>>); N]) -> Self {
        let re = Regex::new(r"^(?:.*::)?(.+)$").unwrap();
        let raw_type_name = std::any::type_name::<Self>();
        let id = match re.captures(raw_type_name) {
            Some(caps) => caps.get(1).map_or(raw_type_name, |v| v.as_str()),
            None => raw_type_name,
        };
        Self {
            id: id.to_owned(),
            values: IndexMap::from(values.map(|(key, value)| (key.to_owned(), value))),
        }
    }
}
//
//
impl<T> NestedValue<T> for MultiValue<T> {
    //
    //
    fn id(&self) -> String {
        self.id.clone()
    }
    //
    //
    fn get(&self, key: &str) -> T {
        let mut keys = key.split('/');
        let key = keys.next().unwrap();
        // println!("{}.get | -> key: {}", self.id, key);
        match self.values.get(key) {
            Some(node) => {
                let key: String = keys.map(|v| format!("{}/", v)).collect();
                // println!("{}.get | key -> : {}", self.id, key);
                node.get(&key)
            }
            None => panic!("{}.get | Not found key '{}'", self.id, key),
        }
    }
    //
    //
    fn store(&mut self, key: &str, value: T) {
        let mut keys = key.split('/');
        let key = keys.next().unwrap();
        // println!("{}.store | -> key: {}", self.id, key);
        match self.values.get_mut(key) {
            Some(node) => {
                let key: String = keys.map(|v| format!("{}/", v)).collect();
                // println!("{}.store | key -> : {}", self.id, key);
                node.store(&key, value)
            }
            None => panic!("{}.store | Not found key '{}'", self.id, key),
        }
    }
}
