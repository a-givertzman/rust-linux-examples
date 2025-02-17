use chrono::DateTime;

use super::bool::Bool;


#[derive(Clone, Debug)]
pub struct Point<T> {
    pub name: String,
    pub value: T,
    pub status: u8,
    pub timestamp: DateTime<chrono::Utc>,
}


impl<T: std::ops::Add<Output = T> + Clone> std::ops::Add for Point<T> {
    type Output = Point<T>;
    fn add(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let timestamp = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => rhs.timestamp,
            std::cmp::Ordering::Equal => self.timestamp,
            std::cmp::Ordering::Greater => self.timestamp,
        };
        Point {
            name: String::from("Point.Add"),
            value: self.value + rhs.value,
            status: status,
            timestamp: timestamp,
        }
    }
}


impl<T: std::ops::BitOr<Output = T>> std::ops::BitOr for Point<T> {
    type Output = Point<T>;
    fn bitor(self, rhs: Self) -> Self::Output {
        let status = match self.status.cmp(&rhs.status) {
            std::cmp::Ordering::Less => rhs.status,
            std::cmp::Ordering::Equal => self.status,
            std::cmp::Ordering::Greater => self.status,
        };
        let timestamp = match self.timestamp.cmp(&rhs.timestamp) {
            std::cmp::Ordering::Less => rhs.timestamp,
            std::cmp::Ordering::Equal => self.timestamp,
            std::cmp::Ordering::Greater => self.timestamp,
        };
        Point {
            name: String::from("Point.BitOr"),
            value: self.value | rhs.value,
            status: status,
            timestamp: timestamp,
        }        
    }
}


///
/// enum container for Point<T>
/// - supported types: Bool, Int, Float
#[derive(Debug, Clone)]
pub enum PointType {
    Bool(Point<Bool>),
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
    pub fn asBool(&self) -> Point<Bool> {
        match self {
            PointType::Bool(point) => point.clone(),
            _ => panic!("PointType.asBool | Invalid point type Bool"),
        }
    }
    pub fn asInt(&self) -> Point<i64> {
        match self {
            PointType::Int(point) => point.clone(),
            _ => panic!("PointType.asInt | Invalid point type Int"),
        }
    }
    pub fn asFloat(&self) -> Point<f64> {
        match self {
            PointType::Float(point) => point.clone(),
            _ => panic!("PointType.asFloat | Invalid point type Float"),
        }
    }
}

