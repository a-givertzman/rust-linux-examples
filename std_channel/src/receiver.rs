use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc, Arc}, thread::{self, JoinHandle}, time::Duration};

use crate::event::Event;

pub struct Receiver {
    index: usize,
    limit: usize,
    recv: Option<mpsc::Receiver<Event>>,
    pub received: Arc<AtomicUsize>,
    exit: Arc<AtomicBool>,
}
impl Receiver {
    pub fn new(index: usize, recv: mpsc::Receiver<Event>, limit: usize) -> Self {
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
            loop {
                match recv.recv_timeout(Duration::from_millis(1000)) {
                    Ok(event) => {
                        let _ = event;
                        received.fetch_add(1, Ordering::SeqCst);      
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
            log::info!("Receiver({}).run | Exit", index);
        })
    }
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}