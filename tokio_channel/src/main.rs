mod event;
mod mqueue;
mod producer;
mod receiver;
mod value;

use std::sync::atomic::Ordering;

use event::Event;
use mqueue::MQueue;
use producer::Producer;
use receiver::Receiver;
use tokio::{task::JoinHandle, time::Instant};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let count = 300_000;
    let receivers = 5;
    let producers = 7;
    let total_produced = count * producers;
    let data: Vec<Event> = (0..count).map(|i| Event {
        name: i.to_string(),
        value: value::Value::Double(0.5),
    }).collect();
    let target_total_received = count * producers * receivers;
    let mut mq = MQueue::new();
    let mut receivers: Vec<Receiver> = (0..receivers).map(|i| Receiver::new(i, mq.subscribe(), count * producers)).collect();
    let mut producers: Vec<Producer> = (0..producers).map(|i| Producer::new(i, mq.link(), &data)).collect();
    let total_time = Instant::now();
    let mq_h = mq.run();
    let r_h: Vec<JoinHandle<()>> = receivers.iter_mut().map(|r| r.run()).collect();
    log::info!("main | {} receivers executed ", receivers.len());
    let p_h: Vec<JoinHandle<()>> = producers.iter_mut().map(|p| p.run()).collect();
    log::info!("main | {} producers executed ", producers.len());
    for h in r_h {
        h.await.unwrap();
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
        h.await.unwrap();
    }
    log::info!("main | {} producers exited ", receivers.len());
    mq.exit();
    mq_h.await.unwrap();
    log::info!("main | MQueue exited ");
    log::info!("main | ---------------------------");
    log::info!("main | All done ");
    log::info!("main | Events: {:?}", data.len());
    log::info!("main | ---------------------------");
    log::info!("main | Producers: {:?}", producers.len());
    log::info!("main | Total produced: {:?}", total_produced);
    log::info!("main | ---------------------------");
    log::info!("main | Receivers: {:?}", receivers.len());
    log::info!("main | Total received: {:?}", total_received);
    log::info!("main | ---------------------------");
    log::info!("main | Total elapsed: {:?}", total_elapsed);
}
