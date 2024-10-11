use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc, RwLock}, thread::{self, JoinHandle}, time::Duration};
use rand::Rng;
use thread_priority::{set_current_thread_priority, NormalThreadSchedulePolicy, RealtimeThreadSchedulePolicy, ThreadBuilderExt, ThreadExt, ThreadPriority, ThreadPriorityOsValue, ThreadSchedulePolicy};

use crate::service_cycle::ServiceCycle;
///
/// Global static counter of CyclicService instances
pub static COUNT: AtomicUsize = AtomicUsize::new(1);
///
/// The service executing with precise cycle time in the separate thread
pub struct CyclicService {
    log_id: String,
    cycle: Duration,
    data: Vec<f64>,
    out: Arc<RwLock<Vec<f64>>>,
    exit: Arc<AtomicBool>,
}
//
//
impl CyclicService {
    ///
    /// 
    pub fn new(cycle: Duration) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            log_id: format!("CyclicService-{:0>4}", COUNT.fetch_add(1, Ordering::Relaxed)),
            cycle,
            data: (0..1000).map(|val| (val as f64) * (rng.gen_range(0..100) as f64) / 100.0).collect(),
            out: Arc::new(RwLock::new(vec![])),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> JoinHandle<()> {
        let log_id = self.log_id.clone();
        let mut cycle = ServiceCycle::new(&self.log_id, self.cycle);
        let exit = self.exit.clone();
        let data = self.data.clone();
        let out = self.out.clone();
        let builder = thread::Builder::new().name("CyclicService | ".to_owned());
        let handle: thread::JoinHandle<_> = builder.spawn(move || {
            log::debug!("{}.run | Start", log_id);
            let result = thread::current().set_priority_and_policy(
                // ThreadSchedulePolicy::Normal(NormalThreadSchedulePolicy::Batch),
                ThreadSchedulePolicy::Realtime(RealtimeThreadSchedulePolicy::Fifo),
                ThreadPriority::Crossplatform(10.try_into().unwrap()),       // Realtime: 1..99; Normal: -20..+19
            );
            log::debug!("{}.run | Set Priority & Policy result: {:?}", log_id, result);
            let priority = thread::current().get_priority();
            let policy = thread::current().get_schedule_policy();
            log::debug!("{}.run | Shedule policy: {:?}", log_id, policy);
            log::debug!("{}.run | ThreadPriority: {:?}", log_id, priority);
            let mut cycles = 0;
            loop {
                cycle.start();
                for val in &data {
                    let r = val * 1000.0 + val * val * 100.0 + val * val * val * 10.0;
                    out.write().unwrap().push(r);
                }
                cycle.wait();
                cycles += 1;
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::debug!("{}.run | Cycles: {}", log_id, cycles);
            log::debug!("{}.run | Exit", log_id);
        }).unwrap();
        handle
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
        log::debug!("{}.exit | Exit: {}", self.log_id, self.exit.load(Ordering::SeqCst));
    }
}