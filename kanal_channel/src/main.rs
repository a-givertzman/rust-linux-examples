mod cycle;
mod event;
mod load;
mod mqueue;
mod producer;
mod receiver;
mod value;

use std::{sync::atomic::Ordering, thread::JoinHandle, time::{Duration, Instant}};

use event::Event;
use load::Load;
use mqueue::MQueue;
use producer::Producer;
use receiver::Receiver;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub events: usize,
    pub receivers: usize,
    pub producers: usize,
    pub loads: usize,
    pub load_interval: u64,
}

///
/// 
fn main() {
    unsafe { std::env::set_var("RUST_LOG", "info") };
    env_logger::init();
    let path = "config.yaml";
    let rdr = std::fs::OpenOptions::new().read(true).open(path).unwrap();
    let config: Config = serde_yaml::from_reader(rdr).unwrap();
    let count = config.events;   // per producer 300_000;
    let receivers = config.receivers;
    let producers = config.producers;
    let loads = config.loads;
    let load_interval = Duration::from_millis(config.load_interval);
    let total_produced = count * producers;
    let data: Vec<Event> = (0..count).map(|i| Event {
        name: i.to_string(),
        value: value::Value::Double(0.5),
    }).collect();
    let target_total_received = count * producers * receivers;
    let mut mq = MQueue::new();
    let mut receivers: Vec<Receiver> = (0..receivers).map(|i| Receiver::new(i, mq.subscribe(), count * producers)).collect();
    let mut producers: Vec<Producer> = (0..producers).map(|i| Producer::new(i, mq.link(), &data)).collect();
    let mut loads: Vec<Load> = (0..loads).map(|i| Load::new(i, load_interval)).collect();
    let total_time = Instant::now();
    let mq_h = mq.run();
    let load_h: Vec<JoinHandle<()>> = loads.iter_mut().map(|l| l.run()).collect();
    log::info!("main | {} loads executed ", loads.len());
    let r_h: Vec<JoinHandle<()>> = receivers.iter_mut().map(|r| r.run()).collect();
    log::info!("main | {} receivers executed ", receivers.len());
    let p_h: Vec<JoinHandle<()>> = producers.iter_mut().map(|p| p.run()).collect();
    log::info!("main | {} producers executed ", producers.len());
    for h in r_h {
        h.join().unwrap();
    }
    let total_elapsed = total_time.elapsed();
    let total_received = receivers.iter().fold(0, |acc, r| {
        let received = r.received.load(Ordering::SeqCst);
        assert!(total_produced == received);
        acc + received
    });
    assert!(target_total_received == total_received, "\ntarget: {target_total_received} \nresult: {total_received}");
    log::info!("main | {} receivers exited ", receivers.len());
    for h in p_h {
        h.join().unwrap();
    }
    log::info!("main | {} producers exited ", receivers.len());
    loads.iter().for_each(|l| l.exit());
    for h in load_h {
        h.join().unwrap();
    }
    log::info!("main | {} loads exited ", loads.len());
    mq.exit();
    mq_h.join().unwrap();
    log::info!("main | kanal channel ");
    log::info!("main | ---------------------------");
    log::info!("main | Events: {:?}", data.len());
    log::info!("main | ---------------------------");
    log::info!("main | Producers: {:?}", producers.len());
    log::info!("main | Total produced: {:?}", total_produced);
    log::info!("main | ---------------------------");
    log::info!("main | Receivers: {:?}", receivers.len());
    log::info!("main | Total received: {:?}", total_received);
    log::info!("main | ---------------------------");
    log::info!("main | Loads: {:?}", loads.len());
    log::info!("main | ---------------------------");
    log::info!("main | Total elapsed: {:?}", total_elapsed);
}
