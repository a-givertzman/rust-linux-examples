use tokio::task::JoinHandle;
use std::{sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc}, time::Duration};

use crate::event::Event;
///
/// 
pub struct MQueue {
    send: mpsc::Sender<Event>,
    subscriptions: Vec<mpsc::Sender<Event>>,
    recv: Option<mpsc::Receiver<Event>>,
    exit: Arc<AtomicBool>,
}
//
//
impl MQueue {
    ///
    /// 
    pub fn new() -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            send,
            subscriptions: Vec::new(),
            recv: Some(recv),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn link(&self) -> mpsc::Sender<Event> {
        self.send.clone()
    }
    ///
    /// 
    pub fn subscribe(&mut self) -> mpsc::Receiver<Event> {
        let (send, recv) = mpsc::channel();
        self.subscriptions.push(send);
        recv
    }
    ///
    /// 
    pub fn run(&mut self) -> JoinHandle<()> {
        let subscriptions: Vec<mpsc::Sender<Event>> = self.subscriptions.drain(0..).collect();
        let recv = self.recv.take().unwrap();
        let exit = self.exit.clone();
        let h = tokio::task::spawn_blocking(move|| {
            log::info!("MQueue.run | Start");
            loop {
                match recv.recv_timeout(Duration::from_millis(300)) {
                    Ok(event) => {
                        for send in &subscriptions {
                            if let Err(err) = send.send(event.clone()) {
                                log::warn!("MQueue.run | Send error: {:?}", err);
                            }
                        }
                    }
                    Err(err) => match err {
                        mpsc::RecvTimeoutError::Timeout => std::thread::sleep(Duration::from_millis(10)),
                        // mpsc::RecvTimeoutError::Timeout => tokio::time::sleep(Duration::from_millis(10)).await,
                        mpsc::RecvTimeoutError::Disconnected => panic!("MQueue.run | Error: {:?}", err),
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("MQueue.run | Exit");
            // if let Err(err) = tokio::task::spawn_blocking(move|| {
            // }).await {
            //     log::error!("MQueue.run | spawn_blocking error: {:?}", err);
            // }
        });
        log::info!("MQueue.run | Start - Ok");
        h
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}