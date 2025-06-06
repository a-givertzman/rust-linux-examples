use num::{Num, Zero};

///
/// The pair of contiguous indexes, between which the value found
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pair<T> {
    pub lower: usize,
    pub upper: usize,
    /// Ratio of requested value between lower and upper 
    pub ratio: T,
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
            ratio: T::zero(),
            val: T::zero(),
        }
    }
    ///
    /// Returns [Pair] new instance
    pub fn with(lower: usize, upper: usize, val: T, ratio: T) -> Self {
        Self {
            lower,
            upper,
            ratio,
            val,
        }
    }
    ///
    /// Returns true if other intersects with
    fn intersects(&self, other: Self) -> bool {
        if (self.lower < other.lower) & (self.upper <= other.lower){
            return false
        }
        if (self.lower >= other.upper) & (self.upper > other.upper){
            return false
        }
        true
        // self.lower == other.lower &&
        // self.upper == other.upper
    }
    ///
    /// Returns the ratio for intarpolation
    pub fn ratio(lo: T, up: T, val: T) -> T {
        if val == lo {
            return T::zero();
        }
        if val == up {
            return T::one();
        }
        (val - lo) / (up - lo)
    }
    ///
    /// Returns lineary interpolated value
    pub fn interpolate(lo: T, up: T, ratio: T) -> T {
        ratio * (up - lo) + lo
    }
}

#[cfg(test)]

mod pair {
    use std::{sync::Once, time::{Duration, Instant}};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::pair::Pair;
    ///
    ///
    static INIT: Once = Once::new();
    ///
    /// once called initialisation
    fn init_once() {
        INIT.call_once(|| {
            // implement your initialisation code to be called only once for current test file
        })
    }
    ///
    /// returns:
    ///  - ...
    fn init_each() -> () {}
    ///
    /// Testing `intersect`
    #[test]
    fn intersects() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("field_extremums");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data: [(i32, Pair<f64>, Pair<f64>, bool); 2] = [
            (01, Pair::new(0, 1), Pair::new(0, 1), true),
            (02, Pair::new(0, 3), Pair::new(1, 2), true),

        ];
        for (step, first, other, target) in test_data {
            let time = Instant::now();
            let result = first.intersects(other);
            // log::debug!("step {}  elapsed: {:?} \nresult: {:?}\ntarget: {:?}", step, time.elapsed(), field.extremums, target);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}