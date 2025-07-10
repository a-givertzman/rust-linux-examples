use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(u64),
    Double(f64),
    String(String),
}
