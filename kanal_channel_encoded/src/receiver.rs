use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc}, thread::{self, JoinHandle}, time::Duration};

use crate::event::Event;

pub struct Receiver {
    index: usize,
    limit: usize,
    recv: Option<kanal::Receiver<Vec<u8>>>,
    pub received: Arc<AtomicUsize>,
    exit: Arc<AtomicBool>,
}
impl Receiver {
    pub fn new(index: usize, recv: kanal::Receiver<Vec<u8>>, limit: usize) -> Self {
        Self {
            index,
            limit,
            recv: Some(recv),
            received: Arc::new(AtomicUsize::new(0)),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn run(&mut self) -> JoinHandle<()> {
        let index = self.index;
        let limit = self.limit;
        let recv = self.recv.take().unwrap();
        let received = self.received.clone();
        let exit = self.exit.clone();
        thread::spawn(move|| {
            log::info!("Receiver({}).run | Start", index);
            let config = bincode::config::standard();
            loop {
                match recv.recv_timeout(Duration::from_millis(1000)) {
                    Ok(event) => {
                        match bincode::decode_from_slice(&event, config) {
                            Ok((event, len)) => {
                                let _ = len;
                                let _: Event = event;
                                received.fetch_add(1, Ordering::SeqCst);      
                            }
                            Err(err) => log::error!("Receiver({}).run | Event decode error: {:?}", index, err),
                        }
                        if received.load(Ordering::SeqCst) >= limit {
                            break;
                        }                  
                    }
                    Err(err) => {
                        panic!("Receiver({}).run | Received: {}, Error: {:?}", index, received.load(Ordering::SeqCst), err);
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("Receiver({}).run | Exit, Received: {}", index, received.load(Ordering::SeqCst));
        })
    }
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}