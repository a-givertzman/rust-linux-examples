use indexmap::IndexMap;
use num::Num;
use crate::field::Field;

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

}