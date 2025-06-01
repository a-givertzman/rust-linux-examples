///
/// The pair of contiguous indexes, between which the value found
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair {
    pub lower: usize,
    pub upper: usize,
}
//
//
impl Pair {
    ///
    /// Returns [Pair] new instance
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            lower,
            upper,
        }
    }
}