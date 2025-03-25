use std::{time::{Duration, Instant}, thread};
///
/// ServiceCycle - provides exact time interval in ms / us (future posible implementation)
///  - creates with Duration of interval
///  - method start() - begins countdown
///  - method wait() - awaiting remainder of the specified interval if not elapsed
/// 
/// [How to sleep for a few microseconds](https://stackoverflow.com/questions/4986818/how-to-sleep-for-a-few-microseconds)
pub struct ServiceCycle {
    id: String,
    instant: Instant,
    interval: Duration,
}
//
// 
impl ServiceCycle {
    ///
    /// Creates ServiceCycle with Duration of interval
    pub fn new(parent: &str, interval: Duration) ->Self {
        Self {
            id: format!("{}/ServiceCycle", parent),
            instant: Instant::now(),
            interval,
        }
    }
    ///
    /// Returns the specified cycle interval
    #[allow(unused)]
    pub fn interval(&self) -> Duration {
        self.interval
    }
    ///
    /// Starts new timer
    pub fn start(&mut self) {
        self.instant = Instant::now();
    }
    ///
    /// Waits for the remaining time,
    /// If the time elapsed since the start
    /// less then the specified cycle interval
    pub async fn wait(&self) {
        let elapsed = self.instant.elapsed();
        if elapsed <= self.interval {
            let remainder = self.interval - elapsed;
            log::trace!("{}.wait | waiting: {:?}", self.id, remainder);
            tokio::time::sleep(remainder).await;
        } else {
            log::error!("{}.wait | exceeded {:?} by {:?}, elapsed {:?}", self.id, self.interval, elapsed - self.interval, elapsed);
        }
    }
    ///
    /// Returns current elapsed time
    pub fn elapsed(&mut self) ->Duration {
        self.instant.elapsed()
    }
}