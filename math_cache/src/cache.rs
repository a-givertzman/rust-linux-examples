use std::fmt::Display;

use indexmap::IndexMap;
use num::Num;
use sal_core::dbg::Dbg;
use crate::field::Field;
///
/// 
pub struct Cache<T> {
    dbg: Dbg,
    fields: IndexMap<String, Field<T>>,
}
//
//
impl<T: Num + PartialOrd + Copy + Display> Cache<T> {
    ///
    /// Returns [Field] new instance
    pub fn new(parent: impl Into<String>, fields: IndexMap<String, Vec<T>>) -> Self {
        let dbg = Dbg::new(parent, "Cache");
        let fields = fields.into_iter().map(|(key, values)| {
            (key, Field::new(&dbg, values))
        }).collect();
        Self {
            dbg,
            fields,
        }
    }
    ///
    /// Returns the row, associated with specivied arguments
    pub fn get(&self, args: IndexMap<String, T>) -> IndexMap<String, T> {
        let mut pairs = IndexMap::new();
        let mut result = IndexMap::new();
        for (key, val) in args {
            match self.fields.get(&key) {
                Some(field) => {
                    let pair = field.get(val);
                    pairs.insert(key, pair);
                }
                None => panic!("Cache.get | Requested key `{key}` - is not found"),
            }
        }
        result
    }
}