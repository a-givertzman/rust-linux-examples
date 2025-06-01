use std::cmp::Ordering;
use num::Num;
use crate::pair::Pair;

///
/// Stores indexed values for cache
pub struct Field<T> {
    extremums: Vec<Pair>,
    sequences: Vec<Vec<T>>,
    values: Vec<T>,
}
//
//
impl<T: Num + PartialOrd + Copy> Field<T> {
    ///
    /// Returns [Field] new instance
    pub fn new(values: Vec<T>) -> Self {
        let (sequences, extremums) = Self::extremums(values.clone(), T::zero());
        Self {
            extremums,
            sequences,
            values,
        }
    }
    ///
    /// Returns [Bound]'s of indexes between which the requested value found
    pub fn get(&self, val: T) -> Vec<Pair> {
        let result = vec![];
        for pair in &self.extremums {

        }
        result
    }
    ///
    /// Returns the extremums of the input sequence,
    /// based on the sign of the first differential
    fn extremums(values: Vec<T>, zero: T) -> (Vec<Vec<T>>, Vec<Pair>) {
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
        let dbg = Dbg::own("field_test");
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
            let field = Field::new(vals);
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
}