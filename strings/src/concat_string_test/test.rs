use std::time::Instant;
use concat_string::concat_string;
use crate::{Error, Test, TestResult};

pub struct ConcatStringTest {
    name: String,
}
impl ConcatStringTest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("ConcatString {}", name.into()),
        }
    }
}
impl Test for ConcatStringTest {
    fn run(&self, count: usize) -> Result<TestResult, Error> {
        log::debug!("ConcatTest.run | count: {}", count);
        let total_time = Instant::now();
        for _ in 0..count {
            let res = concat_string!("vUvVKPR", "Jv", "3", "Mh", "561", "lfS", "AIwLMQDDT", "ld", "0", "WGMEmlXsKEx", "3", "dcjJXQbEKI", "7", "kiz", "4KFY", "9", "CWGOZIDHqSgyIIF", "1", "iXgbvxO", "04", "byO", "08", "Q", "86", "fpFkm", "7", "mu", "03", "crpRMUsfUjEHfH", "8", "r", "92", "vc7LkDnzLOyljXzdeMPqzjLAPb", "1234", "zFuaP", "187365", "OmkhqmmzwgJEuaSK", "80", "DTWBxWpdbGxNSOlKS", "9", "eKkOBhpA5NepJMSng", "4", "J", "23", "hPVqAjD", "9", "LsByYqo", "91", "aDozBpO1G", "4", "mIqkAPWvyVBNRpGIca");
            log::trace!("ConcatTest.run | res: {}", res)
        }
        let total_elapsed = total_time.elapsed();
        Ok(TestResult {
            name: self.name.clone(),
            events: count,
            total_elapsed,
        })
    }
}