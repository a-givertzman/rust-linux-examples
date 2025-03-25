use crate::value::Value;

#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub value: Value,
}

unsafe impl Send for Event {}