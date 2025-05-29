use log_macro::dbg;

fn main() {
    unsafe { std::env::set_var("RUST_LOG", "trace"); }
    env_logger::init();
    let service = Service::new();
    service.run();
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
}