use std::{fmt::Debug, hash::Hash};
use indexmap::IndexMap;
///
/// Container storing several basic types, array and map
#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    String(String),
    Vec(Vec<Value>),
    Map(IndexMap<Value, Value>),
    Null
}
//
//
impl Value {
    ///
    /// Returns containing value if bool else panic
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Bool(value) => *value,
            _ => panic!("Value.as_bool | Expected contained value of type bool, but '{}' found", std::any::type_name::<Self>())
        }
    }
    ///
    /// Returns containing value if i64 else panic
    pub fn as_i64(&self) -> i64 {
        match self {
            Value::I64(value) => *value,
            _ => panic!("Value.as_bool | Expected contained value of type i64, but '{}' found", std::any::type_name::<Self>())
        }
    }
    ///
    /// Returns containing value if u64 else panic
    pub fn as_u64(&self) -> u64 {
        match self {
            Value::U64(value) => *value,
            _ => panic!("Value.as_bool | Expected contained value of type u64, but '{}' found", std::any::type_name::<Self>())
        }
    }
    ///
    /// Returns containing value if f64 else panic
    pub fn as_f64(&self) -> f64 {
        match self {
            Value::F64(value) => *value,
            _ => panic!("Value.as_bool | Expected contained value of type f64, but '{}' found", std::any::type_name::<Self>())
        }
    }
    ///
    /// Returns containing value if String else panic
    pub fn as_str<'a>(&'a self) -> &'a str {
        match self {
            Value::String(value) => value,
            _ => panic!("Value.as_bool | Expected contained value of type String, but '{}' found", std::any::type_name::<Self>())
        }
    }
    ///
    /// Returns containing value if Vec else panic
    pub fn as_vec(&self) -> &Vec<Value> {
        match self {
            Value::Vec(value) => value,
            _ => panic!("Value.as_bool | Expected contained value of type Vec, but '{}' found", std::any::type_name::<Self>())
        }
    }
    ///
    /// Returns containing value if Map else panic
    pub fn as_map(&self) -> &IndexMap<Value, Value> {
        match self {
            Value::Map(value) => value,
            _ => panic!("Value.as_bool | Expected contained value of type Map, but '{}' found", std::any::type_name::<Self>())
        }
    }
}
//
//
impl Hash for Value {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Value::Bool(value) => value.hash(state),
            Value::I64(value) => value.hash(state),
            Value::U64(value) => value.hash(state),
            Value::F64(value) => value.to_string().hash(state),
            Value::String(value) => value.hash(state),
            Value::Vec(value) => value.hash(state),
            Value::Map(_) => panic!("Value.hash | Map can't be used as index"),
            Value::Null => panic!("Value.hash | Null can't be used as index"),
        }
        ;
    }
}
//
//
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::I64(l0), Self::I64(r0)) => l0 == r0,
            (Self::U64(l0), Self::U64(r0)) => l0 == r0,
            (Self::F64(l0), Self::F64(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Vec(l0), Self::Vec(r0)) => l0 == r0,
            (Self::Map(l0), Self::Map(r0)) => {
                for (l_key, l_value) in l0 {
                    match r0.get(l_key) {
                        Some(r_value) => if l_value != r_value {
                            return false;
                        },
                        None => return false,
                    }
                }
                true
            }
            _ => false,
        }
    }
}
//
//
impl Eq for Value {}
//
//
impl Into<bool> for Value {
    fn into(self) -> bool {
        self.as_bool()
    }
}
//
//
impl Into<u64> for Value {
    fn into(self) -> u64 {
        self.as_u64()
    }
}
//
//
impl Into<i64> for Value {
    fn into(self) -> i64 {
        self.as_i64()
    }
}
//
//
impl Into<f64> for Value {
    fn into(self) -> f64 {
        self.as_f64()
    }
}
//
//
impl Into<String> for Value {
    fn into(self) -> String {
        self.as_str().to_owned()
    }
}
//
//
impl Into<Vec<Value>> for Value {
    fn into(self) -> Vec<Value> {
        self.as_vec().to_owned()
    }
}
//
//
impl Into<IndexMap<Value, Value>> for Value {
    fn into(self) -> IndexMap<Value, Value> {
        self.as_map().to_owned()
    }
}
//
//
impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}
//
//
impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Value::U64(value)
    }
}
//
//
impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::I64(value)
    }
}
//
//
impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::F64(value)
    }
}
//
//
impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}
//
//
impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}
