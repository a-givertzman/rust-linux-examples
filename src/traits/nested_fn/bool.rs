#[derive(Debug, Clone)]
pub struct Bool(pub bool);
impl std::ops::Add for Bool {
    type Output = Bool;
    fn add(self, rhs: Self) -> Self::Output {
        Bool(self.0 | rhs.0)
    }
}
