use std::{cmp::Ordering, fmt::Display};
use num::Num;
use sal_core::{
    log::{dbg, warn},
    dbg::Dbg,
};
use crate::pair::Pair;

///
/// Stores indexed values for cache
pub struct Field<T> {
    dbg: Dbg,
    extremums: Vec<Pair<T>>,
    sequences: Vec<Vec<T>>,
    values: Vec<T>,
}
//
//
impl<T: Num + PartialOrd + Copy + Display> Field<T> {
    ///
    /// Returns [Field] new instance
    pub fn new(parent: impl Into<String>, values: Vec<T>) -> Self {
        let (sequences, extremums) = Self::extremums(values.clone(), T::zero());
        Self {
            dbg: Dbg::new(parent, "Field"),
            extremums,
            sequences,
            values,
        }
    }
    ///
    /// Returns [Pair]'s contains the requested value
    pub fn get(&self, val: T) -> Vec<Pair<T>> {
        let result = vec![];
        for pair in &self.extremums {
            for sequence in &self.sequences {
                // sequence.binary_search(x)
            }
            // let lower = self.values pair.lower
        }
        result
    }
    ///
    /// Returns interpolated value
    fn interpolation(lower: T, upper: T, val: T) -> T {
        val
    }
    ///
    /// 
    #[dbg()]
    fn binary_search(sequence: &[T], val: T) -> Option<Pair<T>> {
        if let Some(first) = sequence.first() {
            if val <= *first {
                return Some(Pair::with(0, 0, *first));
            }
        }
        if let Some(last) = sequence.last() {
            if val >= *last {
                let len = sequence.len() -1;
                return Some(Pair::with(len, len, *last));
            }
        }
        let mut len = sequence.len();
        let mut base = 0usize;
        let mut halh;
        while len > 1 {
            halh = len / 2;
            let mid = base + halh;
            log::debug!("Field.binary_search | mid: {mid}");
            match val.partial_cmp(&sequence[mid]) {
                Some(cmp) => {
                    match cmp {
                        Ordering::Less => {
                            if mid > 0 {
                                if val >= sequence[mid - 1] {
                                    return Some(Pair::with(mid -1, mid, val));
                                }
                            }
                            // base = base;
                        }
                        Ordering::Equal => {
                            if mid > 0 {
                                return Some(Pair::with(mid -1, mid, val));
                            } else if mid < (sequence.len() - 1) {
                                return Some(Pair::with(mid, mid +1, val));
                            } else {
                                return Some(Pair::with(mid, mid, val));
                            }
                        }
                        Ordering::Greater => {
                            if mid < (sequence.len() - 1) {
                                if val <= sequence[mid + 1] {
                                    return Some(Pair::with(mid, mid +1, val));
                                }
                            }
                            base = mid;
                        }
                    }
                    len -= halh;
                }
                None => {
                    log::warn!("Field.binary_search | val {val} - is not found");
                    return None;
                }
            }
        }
        None
    }
    ///
    /// Returns the extremums of the input sequence,
    /// based on the sign of the first differential
    fn extremums(values: Vec<T>, zero: T) -> (Vec<Vec<T>>, Vec<Pair<T>>) {
        let mut pairs = vec![];
        let mut result = vec![];
        let mut prev: Option<T> = None;
        let mut prev_sign = None;
        let mut sequence = vec![];
        let mut lower = 0;
        let mut upper = 0;
        for (idx, val) in values.iter().enumerate() {
            if let Some(prev) = prev {
                let sign = Self::sign(*val - prev, zero);
                if let Some(prev_sign) = prev_sign {
                    if (prev_sign != Ordering::Equal) & (sign != Ordering::Equal) {
                        if prev_sign != sign {
                            pairs.push(Pair::new(lower, upper));
                            lower = idx;
                            result.push(sequence);
                            sequence = vec![];
                        }
                    }
                }
                prev_sign = Some(sign);
            }
            upper = idx;
            prev = Some(*val);
            sequence.push(*val);
        }
        if !sequence.is_empty() {
            pairs.push(Pair::new(lower, upper));
            result.push(sequence);
        }
        (result, pairs)
    }
    ///
    /// Returns a sign of the specified value
    fn sign(val: T, zero: T) -> Ordering {
        if val == zero {
            Ordering::Equal
        } else if val > zero {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

#[cfg(test)]

mod field {
    use std::{sync::Once, time::{Duration, Instant}};
    use sal_core::dbg::Dbg;
    use testing::stuff::max_test_duration::TestDuration;
    use debugging::session::debug_session::{DebugSession, LogLevel, Backtrace};
    use crate::{field::Field, pair::Pair};
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
    /// Testing `extremums`
    #[test]
    fn extremums() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("field_extremums");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]],
                vec![Pair::new(0, 7)],
            ),
            (02,
                vec![-0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]],
                vec![Pair::new(0, 7)],
            ),
            (03,
                vec![-0.1, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![vec![-0.1, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7]],
                vec![Pair::new(0, 7)],
            ),
            (04,
                vec![-0.1, -0.1, -0.2, -0.3, -0.4, -0.5, -0.6, 0.7],
                vec![vec![-0.1, -0.1, -0.2, -0.3, -0.4, -0.5, -0.6], vec![0.7]],
                vec![Pair::new(0, 6), Pair::new(7, 7)],
            ),
            (04,
                vec![0.1, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, -0.7],
                vec![vec![0.1, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6], vec![-0.7]],
                vec![Pair::new(0, 6), Pair::new(7, 7)],
            ),
            (04,
                vec![0.1, 0.1, -0.2, -0.3, -0.4, -0.5, -0.6, 0.7],
                vec![vec![0.1, 0.1, -0.2, -0.3, -0.4, -0.5, -0.6], vec![0.7]],
                vec![Pair::new(0, 6), Pair::new(7, 7)],
            ),
            (05,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, -0.8],
                vec![vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7], vec![-0.8]],
                vec![Pair::new(0, 7), Pair::new(8, 8)],
            ),
            (06,
                vec![0.0, 0.1, 0.2, 0.3, 0.2, 0.1, 0.0, -0.1],
                vec![vec![0.0, 0.1, 0.2, 0.3], vec![0.2, 0.1, 0.0, -0.1]],
                vec![Pair::new(0, 3), Pair::new(4, 7)],
            ),
            (07,
                vec![0.0, -0.1, -0.2, -0.1, 0.0, 0.1, 0.0, -0.1],
                vec![vec![0.0, -0.1, -0.2], vec![-0.1, 0.0, 0.1], vec![0.0, -0.1]],
                vec![Pair::new(0, 2), Pair::new(3, 5), Pair::new(6, 7)],
            ),
            (08,
                vec![0.0],
                vec![vec![0.0]],
                vec![Pair::new(0, 0)],
            ),
            (09,
                vec![0.0, 0.1],
                vec![vec![0.0, 0.1]],
                vec![Pair::new(0, 1)],
            ),
            (10,
                vec![0.0, -0.1],
                vec![vec![0.0, -0.1]],
                vec![Pair::new(0, 1)],
            ),
        ];
        for (step, vals, target_sequences, target_extermums) in test_data {
            let time = Instant::now();
            let field = Field::new(&dbg, vals);
            let result = field.extremums.clone();
            let target = target_extermums;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            let result = field.sequences.clone();
            let target = target_sequences;
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
            log::debug!("step {}  elapsed: {:?} \nresult: {:?}\nresult: {:?}\ntarget: {:?}", step, time.elapsed(), field.extremums, field.sequences, target);
        }
        test_duration.exit();
    }
    ///
    /// Testing `binary_search`
    #[test]
    fn binary_search() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("field_binary_search");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01,
                0.15,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(1, 2, 0.15)),
            ),
            (02,
                0.05,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(0, 1, 0.05)),
            ),
            (03,
                0.01,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(0, 1, 0.01)),
            ),
            (04,
                0.55,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(5, 6, 0.55)),
            ),
            (05,
                0.55,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(5, 6, 0.55)),
            ),
            (06,
                0.65,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(6, 7, 0.65)),
            ),
            (07,
                -0.10,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(0, 0, 0.00)),
            ),
            (08,
                0.70,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(7, 7, 0.70)),
            ),
            (09,
                0.80,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                Some(Pair::with(7, 7, 0.70)),
            ),
        ];
        for (step, val, vals, target) in test_data {
            let time = Instant::now();
            let result = Field::binary_search(&vals, val);
            log::debug!("step {}  elapsed: {:?} \nval: {:?}\nresult: {:?}\ntarget: {:?}", step, time.elapsed(), val, result, target);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }
        test_duration.exit();
    }
}