use bincode::{Decode, Encode};
use crate::Value;

#[derive(Debug, Clone, Decode, Encode)]
pub struct Event {
    pub name: String,
    pub value: Value,
}

unsafe impl Send for Event {}