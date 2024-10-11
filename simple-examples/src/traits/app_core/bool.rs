use std::fmt;

#[derive(Debug, Clone)]
pub struct Bool(pub bool);
impl std::ops::Add for Bool {
    type Output = Bool;
    fn add(self, rhs: Self) -> Self::Output {
        Bool(self.0 | rhs.0)
    }
}

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl std::ops::BitOr for Bool {
    type Output = Bool;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bool(self.0 | rhs.0)
    }
}