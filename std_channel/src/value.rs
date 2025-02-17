#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Int(u64),
    Double(f64),
    String(String),
}