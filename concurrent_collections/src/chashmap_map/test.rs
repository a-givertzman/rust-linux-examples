use std::{sync::Arc, thread::JoinHandle, time::{Duration, Instant}};
use crate::{Error, Event, Load, Test, TestResult};
use super::{Producer, Receiver};

pub struct CHashMapTest {
    name: String,
}
impl CHashMapTest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: format!("CHashMap {}", name.into()),
        }
    }
}
impl Test for CHashMapTest {
    fn run(&self, receivers: usize, producers: usize, loads: usize, load_interval: Duration, data: Vec<Event>) -> Result<TestResult, Error> {
        let events = data.len();
        let total_produced = events * producers;
        let target_total_received = events * producers;
        let receiver: Arc<Receiver> = Arc::new(Receiver::new(0, events * producers));
        let mut producers: Vec<Producer> = (0..producers).map(|i| Producer::new(i, receiver.clone(), &data)).collect();
        let mut loads: Vec<Load> = (0..loads).map(|i| Load::new(i, load_interval)).collect();
        let total_time = Instant::now();
        let load_h: Vec<JoinHandle<()>> = loads.iter_mut().map(|l| l.run()).collect();
        log::debug!("CHashMapTest.run | {} loads executed ", loads.len());
        let p_h: Vec<JoinHandle<()>> = producers.iter_mut().map(|p| p.run()).collect();
        log::debug!("CHashMapTest.run | {} producers executed ", producers.len());
        for h in p_h {
            h.join().unwrap();
        }
        log::debug!("CHashMapTest.run | {} producers exited ", producers.len());
        let total_elapsed = total_time.elapsed();
        let total_received = receiver.received();
        assert!(target_total_received == total_received, "\ntarget: {target_total_received} \nresult: {total_received}");
        loads.iter().for_each(|l| l.exit());
        for h in load_h {
            h.join().unwrap();
        }
        log::debug!("CHashMapTest.run | {} loads exited ", loads.len());
        Ok(TestResult {
            name: self.name.clone(),
            events,
            producers: producers.len(),
            total_produced,
            receivers,
            total_received,
            loads: loads.len(),
            total_elapsed,
        })
    }
}