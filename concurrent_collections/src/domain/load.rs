use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, JoinHandle}, time::Duration};
use rand::Rng;
use crate::ServiceCycle;
///
/// 
pub struct Load {
    index: usize,
    interval: Duration,
    exit: Arc<AtomicBool>,
}
//
//
impl Load {
    ///
    /// 
    pub fn new(index: usize, interval: Duration) -> Self {
        Self {
            index,
            interval,
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> JoinHandle<()> {
        let index = self.index;
        let dbg = format!("Load({})", index);
        let interval: Duration = self.interval.clone();
        let exit = self.exit.clone();
        let mut rng = rand::rng();
        let p = rng.random_range(10_000..100_000);
        thread::spawn(move|| {
            log::debug!("{}.run | Start", dbg);
            let mut cycle = ServiceCycle::new(&format!("{}", dbg), interval);
            loop {
                cycle.start();
                let _lucas_lehmer = Self::lucas_lehmer(p);
                // log::debug!("{}.run | P: {:?}, lucas_lehmer: {}", dbg, p, lucas_lehmer);
                if exit.load(Ordering::SeqCst) {
                    break;
                }
                cycle.wait();
            }
            log::debug!("{}.run | Exit", dbg);
        })
    }
    ///
    /// Determine if Mp = 2p − 1 is prime for p > 2
    fn lucas_lehmer(p: i64) -> String {
        assert!(p > 2);
        let mut s = 4;
        let m = 2 * p - 1;
        for _ in 0..(p - 2) {
            s = ((s * s) - 2) % m
        }
        if s == 0 {
            "PRIME".into()
        } else {
            "COMPOSITE".into()
        }
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}