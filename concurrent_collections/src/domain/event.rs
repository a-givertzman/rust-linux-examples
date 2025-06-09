use bincode::{Decode, Encode};
use crate::value::Value;

#[derive(Debug, Clone, Decode, Encode)]
pub struct Event {
    pub name: String,
    pub value: Value,
}

unsafe impl Send for Event {}