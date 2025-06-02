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
    values: Vec<T>,
}
//
//
impl<T: Num + PartialOrd + Copy + Display> Field<T> {
    ///
    /// Returns [Field] new instance
    pub fn new(parent: impl Into<String>, values: Vec<T>) -> Self {
        Self {
            dbg: Dbg::new(parent, "Field"),
            values,
        }
    }
    ///
    /// Returns [Pair]'s contains the requested value
    #[dbg()]
    pub fn get(&self, val: T) -> Vec<Pair<T>> {
        self.search(val)
    }
    ///
    /// Returns [Pair]s containing requested value
    #[dbg()]
    fn search(&self, val: T) -> Vec<Pair<T>> {
        let mut result = vec![];
        // if let Some(first) = self.values.first() {
        //     if val < *first {
        //         result.push(Pair::with(0, 1, *first));
        //     }
        // }
        for idx in 0..(self.values.len() -1) {
            let value = self.values[idx];
            let next = self.values[idx + 1];
            if (val >= value) & (val <= next) | (val <= value) & (val >= next) {
                result.push(
                    Pair::with(idx, idx + 1, val)
                );
            }
        }
        // if let Some(last) = self.values.last() {
        //     if val > *last {
        //         let len = self.values.len() -1;
        //         result.push(Pair::with(len -1, len, *last));
        //     }
        // }
        // log::debug!("Field.binary_search | mid: {mid}");
        result
    }
    ///
    /// Returns value by it's index
    /// 
    /// Unsafe method, be sure index in range
    /// 
    /// ## Panics if index out of range
    pub fn get_by_idx(&self, index: usize) -> T {
        self.values[index]
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
    /// Testing `search`
    #[test]
    fn search() {
        DebugSession::init(LogLevel::Debug, Backtrace::Short);
        init_once();
        init_each();
        let dbg = Dbg::own("field_search");
        log::debug!("\n{}", dbg);
        let test_duration = TestDuration::new(&dbg, Duration::from_secs(10));
        test_duration.run().unwrap();
        let test_data = [
            (01,
                0.00,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![Pair::with(0, 1, 0.00)],
            ),
            (02,
                0.15,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![Pair::with(1, 2, 0.15)],
            ),
            (03,
                0.05,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![Pair::with(0, 1, 0.05)],
            ),
            (04,
                0.01,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![Pair::with(0, 1, 0.01)],
            ),
            (05,
                0.55,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![Pair::with(5, 6, 0.55)],
            ),
            (06,
                0.55,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![Pair::with(5, 6, 0.55)],
            ),
            (07,
                0.65,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![Pair::with(6, 7, 0.65)],
            ),
            // (08,
            //     -0.10,
            //     vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
            //     vec![Pair::with(0, 1, 0.00)],
            // ),
            (09,
                0.70,
                vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
                vec![Pair::with(6, 7, 0.70)],
            ),
            // (10,
            //     0.80,
            //     vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
            //     vec![Pair::with(6, 7, 0.70)],
            // ),
            (11,
                0.15,
                //    0    1    2    3    4    5    6    7    8
                vec![0.0, 0.1, 0.2, 0.1, 0.0, 0.1, 0.2, 0.1, 0.0],
                vec![Pair::with(1, 2, 0.15), Pair::with(2, 3, 0.15), Pair::with(5, 6, 0.15), Pair::with(6, 7, 0.15)],
            ),
        ];
        for (step, val, vals, target) in test_data {
            let time = Instant::now();
            let field = Field::new(&dbg, vals);
            let result = field.search(val);
            log::debug!("step {}  elapsed: {:?} \nval: {:?}\nresult: {:?}\ntarget: {:?}", step, time.elapsed(), val, result, target);
            assert!(result == target, "step {} \nresult: {:?}\ntarget: {:?}", step, result, target);
        }        test_duration.exit();
    }
}