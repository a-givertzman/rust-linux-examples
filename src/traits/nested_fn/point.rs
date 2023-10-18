use chrono::DateTime;


#[derive(Clone, Debug)]
pub struct Point<T> {
    pub name: String,
    pub value: T,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}


#[derive(Debug, Clone)]
pub enum PointType {
    Bool(Point<bool>),
    Int(Point<i64>),
    Float(Point<f64>),
}
impl PointType {
    pub fn name(&self) -> String {
        match self {
            PointType::Bool(point) => point.name.clone(),
            PointType::Int(point) => point.name.clone(),
            PointType::Float(point) => point.name.clone(),
        }
    }
}

