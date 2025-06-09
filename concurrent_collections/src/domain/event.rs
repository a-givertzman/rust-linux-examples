use bincode::{Decode, Encode};
use crate::Value;

#[derive(Debug, Clone, Decode, Encode, PartialEq)]
pub struct Event {
    pub name: String,
    pub value: Value,
}

unsafe impl Send for Event {}
