use tokio::task::JoinHandle;
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

use crate::event::Event;
///
/// 
pub struct MQueue {
    send: flume::Sender<Event>,
    subscriptions: Vec<flume::Sender<Event>>,
    recv: Option<flume::Receiver<Event>>,
    exit: Arc<AtomicBool>,
}
//
//
impl MQueue {
    ///
    /// 
    pub fn new() -> Self {
        let (send, recv) = flume::unbounded();
        Self {
            send,
            subscriptions: Vec::new(),
            recv: Some(recv),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn link(&self) -> flume::Sender<Event> {
        self.send.clone()
    }
    ///
    /// 
    pub fn subscribe(&mut self) -> flume::Receiver<Event> {
        let (send, recv) = flume::unbounded();
        self.subscriptions.push(send);
        recv
    }
    ///
    /// 
    pub fn run(&mut self) -> JoinHandle<()> {
        let subscriptions: Vec<flume::Sender<Event>> = self.subscriptions.drain(0..).collect();
        let recv = self.recv.take().unwrap();
        let exit = self.exit.clone();
        let h = tokio::spawn(async move {
            log::info!("MQueue.run | Start");
            loop {
                match recv.recv() {
                    Ok(event) => {
                        for send in &subscriptions {
                            if let Err(err) = send.send(event.clone()) {
                                log::warn!("MQueue.run | Send error: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        panic!("MQueue.run | Receive error: {:?}", err);
                        // match err {
                        //     flume::RecvTimeoutError::Timeout => {
                        //         // tokio::time::sleep(Duration::from_millis(10)).await;
                        //     }
                        //     flume::RecvTimeoutError::Disconnected => panic!("MQueue.run | Error: {:?}", err),
                        // }
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("MQueue.run | Exit");
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