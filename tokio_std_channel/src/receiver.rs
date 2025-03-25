use std::{sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, mpsc, Arc}, time::Duration};
use tokio::task::JoinHandle;
use crate::event::Event;
///
///
pub struct Receiver {
    index: usize,
    limit: usize,
    recv: Option<mpsc::Receiver<Event>>,
    pub received: Arc<AtomicUsize>,
    exit: Arc<AtomicBool>,
}
//
//
impl Receiver {
    ///
    /// 
    pub fn new(index: usize, recv: mpsc::Receiver<Event>, limit: usize) -> Self {
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
        tokio::task::spawn_blocking(move|| {
            log::info!("Receiver({}).run | Start", index);
            loop {
                match recv.recv_timeout(Duration::from_millis(300)) {
                    Ok(event) => {
                        let _ = event;
                        received.fetch_add(1, Ordering::SeqCst);      
                        if received.load(Ordering::SeqCst) >= limit {
                            exit.store(true, Ordering::SeqCst);
                        }                  
                    }
                    Err(err) => match err {
                        mpsc::RecvTimeoutError::Timeout => std::thread::sleep(Duration::from_millis(10)),
                        // mpsc::RecvTimeoutError::Timeout => tokio::time::sleep(Duration::from_millis(10)).await,
                        mpsc::RecvTimeoutError::Disconnected => {
                            panic!("Receiver({}).run | Received: {}, Error: {:?}", index, received.load(Ordering::SeqCst), err);
                        }
                    }
                };
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("Receiver({}).run | Exit, Received: {}", index, received.load(Ordering::SeqCst));
        })
        // tokio::spawn(async move {
        // })
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}