//!
//! Testing the threads priority 
//! in case of multiple threads with the different load, 
//! when there is a thread(s) whis precise execution cycle
mod cyclic_service;
mod hard_load_service;
mod service_cycle;

use std::{env, fs, io::{BufRead, BufReader}, path::Path, thread::JoinHandle, time::Duration};
use cyclic_service::CyclicService;
use hard_load_service::HardLoadService;
///
/// 
fn main() {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let loads = 20;          // services with hard load
    let cyclics = 5;    // services with precise cycle time

    match load("src/multi_thread/test_data.txt") {
        Ok(test_data) => {
            let mut cyclic_services: Vec<(CyclicService, JoinHandle<()>)> = vec![];
            for _ in 1..=cyclics {
                let mut cyclic_service = CyclicService::new(Duration::from_millis(1));
                let cyclic_service_handle = cyclic_service.run();
                cyclic_services.push((cyclic_service, cyclic_service_handle));
            }
            let mut hard_load_services = vec![];
            for _ in 1..=loads {
                let mut hard_load_service = HardLoadService::new(
                    r"(\d+-\d+-\d+-\d+)",
                    test_data.clone()
                );
                let hard_load_service_handle = hard_load_service.run();
                hard_load_services.push(hard_load_service_handle);
            }
            for handle in hard_load_services {
                handle.join().unwrap();
            }
            for (service, handle) in cyclic_services {
                service.exit();
                handle.join().unwrap();
            }
        }
        Err(err) => log::info!("main.load | Open error: {:#?}", err),
    }
}
///
/// 
fn load(path: impl AsRef<Path>) -> Result<Vec<String>, String> {
    let f = fs::OpenOptions::new()
        .read(true)
        .open(path);
    match f {
        Ok(f) => {
            let buf = BufReader::new(f);
            let lines = buf.lines()
                .fold(vec![], |mut lines, line| {
                    match line {
                        Ok(line) => lines.push(line),
                        Err(err) => log::info!("main.load | Read error: {:#?}", err),
                    };
                    lines
                });
            Ok(lines)
        }
        Err(err) => {
            let msg = format!("main.load | Open error: {:#?}", err);
            log::info!("main.load | Open error: {:#?}", err);
            Err(msg)
        },
    }
}