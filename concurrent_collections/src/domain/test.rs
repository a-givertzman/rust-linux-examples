use std::time::Duration;

use crate::{Error, Event, TestResult};

pub trait Test {
    fn run(&self, receivers: usize, producers: usize, loads: usize, load_interval: Duration, data: Vec<Event>) -> Result<TestResult, Error>;
}