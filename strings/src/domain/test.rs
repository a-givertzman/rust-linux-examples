use crate::{Error, TestResult};

pub trait Test {
    fn run(&self, count: usize) -> Result<TestResult, Error>;
}