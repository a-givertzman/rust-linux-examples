use bincode::{Decode, Encode};

#[derive(Debug, Clone, Decode, Encode)]
pub enum Value {
    Bool(bool),
    Int(u64),
    Double(f64),
    String(String),
}