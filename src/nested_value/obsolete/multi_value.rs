use indexmap::IndexMap;
use crate::nested_value::NestedValue;
///
/// Containing multiple nested values
pub struct MultiValue<T> {
    id: String,
    inited: bool,
    values: IndexMap<String, Box<dyn NestedValue<T>>>
}
//
//
impl<T> MultiValue<T> {
    ///
    /// Returns new instance of the [MultiValue]
    /// - values - array of the pairs 'key' - 'NestedValue' 
    pub fn new<const N: usize>(values: [(&str, Box<dyn NestedValue<T>>); N]) -> Self {
        let id = "".to_owned();
        let mut me = Self {
            id: id.to_owned(),
            inited: false,
            values: IndexMap::from(values.map(|(key, value)| (key.to_owned(), value))),
        };
        me.init(&id);
        me
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
    fn init(&mut self, key: &str) {
        self.id = key.to_owned();
        for (key, node) in &mut self.values {
            node.init(&format!("{}/{}", self.id, key))
        }
        self.inited = true;
    }
    //
    //
    fn get(&self, key: &str) -> Result<T, String> {
        let mut keys = key.split('/');
        let key = keys.next().unwrap();
        // println!("{}.get | -> key: {}", self.id, key);
        match self.values.get(key) {
            Some(node) => {
                let key: String = keys.map(|v| format!("{}/", v)).collect();
                // println!("{}.get | key -> : {}", self.id, key);
                node.get(&key)
            }
            None => Err(format!("{}.get | Not found key '{}'", self.id, key)),
        }
    }
    //
    //
    fn store(&mut self, editor: &str, key: &str, value: T) -> Result<(), String> {
        let mut keys = key.split('/');
        let key = keys.next().unwrap();
        // println!("{}.store | -> key: {}", self.id, key);
        match self.values.get_mut(key) {
            Some(node) => {
                let key: String = keys.map(|v| format!("{}/", v)).collect();
                // println!("{}.store | key -> : {}", self.id, key);
                node.store(editor, &key, value)
            }
            None => Err(format!("{}.store | Not found key '{}'", self.id, key)),
        }
    }
    //
    //
    fn edited(&self, key: &str) -> Result<Vec<String>, String> {
        let mut keys = key.split('/');
        let key = keys.next().unwrap();
        // println!("{}.get | -> key: {}", self.id, key);
        match self.values.get(key) {
            Some(node) => {
                let key: String = keys.map(|v| format!("{}/", v)).collect();
                // println!("{}.get | key -> : {}", self.id, key);
                node.edited(&key)
            }
            None => Err(format!("{}.get | Not found key '{}'", self.id, key)),
        }
    }
}
