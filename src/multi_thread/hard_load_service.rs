use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc, RwLock}, thread::{self, JoinHandle}, time::Duration};
use rand::Rng;
use regex::Regex;
///
/// Global static counter of HardLoadService instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);
///
/// The service executing with hard load task in the separate thread
pub struct HardLoadService {
    log_id: String,
    pattern: String,
    queue: Arc<RwLock<Vec<String>>>,
    out: Arc<RwLock<Vec<Result<String, String>>>>,
}
//
//
impl HardLoadService {
    ///
    /// 
    pub fn new(pattern: impl Into<String>, queue: Vec<impl Into<String>>) -> Self {
        let log_id = format!("HardLoadService-{:0>4}", COUNT.fetch_add(1, Ordering::Relaxed));
        let queue = queue
            .into_iter()
            .map(|item| item.into())
            .collect();
        // log::info!("{}.new | queue: {:#?}", log_id, queue);
        Self {
            log_id,
            pattern: pattern.into(),
            queue: Arc::new(RwLock::new(queue)),
            out: Arc::new(RwLock::new(vec![])),
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> JoinHandle<()> {
        let log_id = self.log_id.clone();
        let re = Regex::new(&self.pattern).unwrap();
        let queue = self.queue.clone();
        let out = self.out.clone();
        let builder = thread::Builder::new().name("HardLoadService | ".to_owned());
        let handle: thread::JoinHandle<_> = builder.spawn(move || {
            log::info!("{}.run | Start", log_id);
            let queue = queue.read().unwrap();
            let mut out = out.write().unwrap();
            let mut parsed = 0;
            let mut rng = rand::thread_rng();
            for item in queue.iter() {
                match re.captures(item) {
                    Some(caps) => match caps.get(1) {
                        Some(matched) => {
                            // log::info!("{}.run | Parsed '{:?}'",log_id, matched);
                            out.push(Ok(matched.as_str().to_owned()));
                            parsed += 1;
                        }
                        None => {
                            let msg = format!("{}.run | Parse error in '{}'",log_id, item);
                            log::info!("{}", msg);
                            out.push(Err(msg));
                        }
                    }
                    None => {
                        let msg = format!("{}.run | Parse error in '{}'", log_id, item);
                        log::warn!("{}", msg);
                        out.push(Err(msg));
                    }
                };
                let dur = rng.gen_range(1..30);
                thread::sleep(Duration::from_millis(dur));
            }
            log::info!("{}.run | Parsed: {}", log_id, parsed);
            log::info!("{}.run | Exit", log_id);
        }).unwrap();
        handle
    }
}