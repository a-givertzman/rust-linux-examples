use log_macro::dbg;
use sal_core::log::{info, debug, trace, warn, error};

fn main() {
    unsafe { std::env::set_var("RUST_LOG", "trace"); }
    env_logger::init();
    let service = Service::new();
    service.run();
    service.start();
}

#[derive(Debug)]
struct Service {
    dbg: String,
}
//
//
impl Service {
    pub fn new() -> Self {
        Self {
            dbg: "Service".into(),
        }
    }
    ///
    /// Logging using log_macro from local
    #[dbg("dbg-custom-name")]
    pub fn run(&self) {
        log_macro::info!("Start");
        for i in 0..3 {
            let ii = i.to_string();
            log_macro::debug!("Start {ii}");
            for j in 0..3 {
                let jj = j.to_string();
                log_macro::trace!("Working on {} {}", ii, jj);
                let jjj = &jj;
                log_macro::warn!("Working on {} {}: {}", ii, jj, jjj);
                log_macro::error!("Working on {} {}: {}", ii, jj, jjj);
            }
        }
        log_macro::info!("Exit");
    }
    ///
    /// Logging using sal-core::log
    #[dbg("dbg-custom-name")]
    pub fn start(&self) {
        info!("Start");
        for i in 0..3 {
            let ii = i.to_string();
            debug!("Start {ii}");
            for j in 0..3 {
                let jj = j.to_string();
                trace!("Working on {} {}", ii, jj);
                let jjj = &jj;
                warn!("Working on {} {}: {}", ii, jj, jjj);
                error!("Working on {} {}: {}", ii, jj, jjj);
            }
        }
        info!("Exit");
    }
}