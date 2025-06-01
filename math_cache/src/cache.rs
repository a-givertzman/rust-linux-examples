use indexmap::IndexMap;
use num::Num;
use crate::field::Field;
///
/// 
pub struct Cache<T> {
    fields: IndexMap<String, Field<T>>,
}
//
//
impl<T: Num + PartialOrd + Copy> Cache<T> {
    ///
    /// Returns [Field] new instance
    pub fn new(fields: IndexMap<String, Vec<T>>) -> Self {
        let fields = fields.into_iter().map(|(key, values)| {
            (key, Field::new(values))
        }).collect();
        Self {
            fields,
        }
    }
    ///
    /// Returns the row, associated with specivied arguments
    pub fn get(&self, args: IndexMap<String, T>) -> IndexMap<String, T> {
        let mut result = IndexMap::new();
        for (key, val) in args {
            match self.fields.get(&key) {
                Some(field) => {
                    let pairs = field.get(val);
                }
                None => panic!("Cache.get | Requested key `{key}` - is not found"),
            }
        }
        result
    }
}