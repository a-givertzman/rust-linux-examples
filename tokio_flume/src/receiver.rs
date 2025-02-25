use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Arc}, time::Duration};
use tokio::task::JoinHandle;
use crate::event::Event;
///
///
pub struct Receiver {
    index: usize,
    limit: usize,
    recv: Option<flume::Receiver<Event>>,
    pub received: Arc<AtomicUsize>,
    exit: Arc<AtomicBool>,
}
//
//
impl Receiver {
    ///
    /// 
    pub fn new(index: usize, recv: flume::Receiver<Event>, limit: usize) -> Self {
        Self {
            index,
            limit,
            recv: Some(recv),
            received: Arc::new(AtomicUsize::new(0)),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> JoinHandle<()> {
        let index = self.index;
        let limit = self.limit;
        let recv = self.recv.take().unwrap();
        let received = self.received.clone();
        let exit = self.exit.clone();
        tokio::spawn(async move {
            log::info!("Receiver({}).run | Start", index);
            loop {
                match recv.try_recv() {
                    Ok(event) => {
                        let _ = event;
                        received.fetch_add(1, Ordering::SeqCst);      
                        if received.load(Ordering::SeqCst) >= limit {
                            break;
                        }                  
                    }
                    Err(err) => {
                        // panic!("Receiver({}).run | Received: {}, Error: {:?}", index, received.load(Ordering::SeqCst), err);
                        match err {
                            flume::TryRecvError::Empty => {
                                tokio::time::sleep(Duration::from_millis(10)).await;
                            }
                            flume::TryRecvError::Disconnected => {
                                panic!("Receiver({}).run | Received: {}, Error: {:?}", index, received.load(Ordering::SeqCst), err);
                            }
                        }
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("Receiver({}).run | Exit, Received: {}", index, received.load(Ordering::SeqCst));
        })
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}