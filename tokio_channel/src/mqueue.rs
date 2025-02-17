use tokio::{sync::mpsc, task::JoinHandle};
use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, time::Duration};

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
        let (send, recv) = mpsc::channel(10_000);
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
        let (send, recv) = mpsc::channel(10_000);
        self.subscriptions.push(send);
        recv
    }
    ///
    /// 
    pub fn run(&mut self) -> JoinHandle<()> {
        let subscriptions: Vec<mpsc::Sender<Event>> = self.subscriptions.drain(0..).collect();
        let mut recv = self.recv.take().unwrap();
        let exit = self.exit.clone();
        let h = tokio::spawn(async move {
            log::info!("MQueue.run | Start");
            loop {
                match recv.try_recv() {
                    Ok(event) => {
                        for send in &subscriptions {
                            if let Err(err) = send.send(event.clone()).await {
                                log::warn!("MQueue.run | Send error: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        match err {
                            mpsc::error::TryRecvError::Empty => {
                                tokio::time::sleep(Duration::from_millis(10)).await;
                            }
                            mpsc::error::TryRecvError::Disconnected => panic!("MQueue.run | Error: {:?}", err),
                        }
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