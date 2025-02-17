use std::{sync::{atomic::{AtomicBool, Ordering}, mpsc::{self}, Arc}, thread::{self, JoinHandle}, time::Duration};

use crate::event::Event;

pub struct MQueue {
    send: mpsc::Sender<Event>,
    subscriptions: Vec<mpsc::Sender<Event>>,
    recv: Option<mpsc::Receiver<Event>>,
    exit: Arc<AtomicBool>,
}
impl MQueue {
    pub fn new() -> Self {
        let (send, recv) = mpsc::channel();
        Self {
            send,
            subscriptions: Vec::new(),
            recv: Some(recv),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn link(&self) -> mpsc::Sender<Event> {
        self.send.clone()
    }
    pub fn subscribe(&mut self) -> mpsc::Receiver<Event> {
        let (send, recv) = mpsc::channel();
        self.subscriptions.push(send);
        recv
    }
    pub fn run(&mut self) -> JoinHandle<()> {
        let subscriptions: Vec<mpsc::Sender<Event>> = self.subscriptions.drain(0..).collect();
        let recv = self.recv.take().unwrap();
        let exit = self.exit.clone();
        thread::spawn(move|| {
            log::info!("MQueue.run | Start");
            loop {
                match recv.recv_timeout(Duration::from_millis(1000)) {
                    Ok(event) => {
                        for send in &subscriptions {
                            send.send(event.clone()).unwrap()
                        }
                    }
                    Err(err) => {
                        panic!("MQueue.run | Error: {:?}", err)
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("MQueue.run | Exit");
        })
    }
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}