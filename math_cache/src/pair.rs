use num::{Num, Zero};

///
/// The pair of contiguous indexes, between which the value found
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair<T> {
    pub lower: usize,
    pub upper: usize,
    /// Ratio of requested value between lower and upper 
    pub ratio: f64,
    /// Calculated by interpolation value
    pub val: T,
}
//
//
impl<T: Num + PartialOrd + Copy + Zero> Pair<T> {
    ///
    /// Returns [Pair] new instance
    pub fn new(lower: usize, upper: usize) -> Self {
        Self {
            lower,
            upper,
            ratio: 0.0,
            val: T::zero(),
        }
    }
    ///
    /// Returns [Pair] new instance
    pub fn with(lower: usize, upper: usize, val: T) -> Self {
        Self {
            lower,
            upper,
            ratio: 0.0,
            val,
        }
    }
}
