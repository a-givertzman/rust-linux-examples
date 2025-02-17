mod event;
mod mqueue;
mod producer;
mod receiver;
mod value;

use std::{thread::JoinHandle, time::Instant};

use event::Event;
use mqueue::MQueue;
use producer::Producer;
use receiver::Receiver;

///
/// 
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let count = 100;
    let receivers = 3;
    let producers = 3;
    let data: Vec<Event> = (1..count).map(|i| Event {
        name: i.to_string(),
        value: value::Value::Double(0.5),
    }).collect();
    let mut mq = MQueue::new();
    let mut receivers: Vec<Receiver> = (0..receivers).map(|i| Receiver::new(i, mq.subscribe(), data.len())).collect();
    let mut producers: Vec<Producer> = (0..producers).map(|i| Producer::new(i, mq.link(), &data)).collect();
    let total_time = Instant::now();
    let mq_h = mq.run();
    let r_h: Vec<JoinHandle<()>> = receivers.iter_mut().map(|r| r.run()).collect();
    log::info!("main | {} receivers executed ", receivers.len());
    let p_h: Vec<JoinHandle<()>> = producers.iter_mut().map(|p| p.run()).collect();
    log::info!("main | {} producers executed ", producers.len());
    for h in r_h {
        h.join().unwrap();
    }
    log::info!("main | {} receivers exited ", receivers.len());
    for p in p_h {
        p.join().unwrap();
    }
    log::info!("main | {} producers exited ", receivers.len());
    mq.exit();
    mq_h.join().unwrap();
    log::info!("main | MQueue exited ");
    log::info!("main | All done ");
    log::info!("main | Total elapsed: {:?}", total_time.elapsed());
}
