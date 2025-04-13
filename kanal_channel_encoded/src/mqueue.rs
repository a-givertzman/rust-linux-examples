use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, JoinHandle}, time::Duration};

pub struct MQueue {
    send: kanal::Sender<Vec<u8>>,
    subscriptions: Vec<kanal::Sender<Vec<u8>>>,
    recv: Option<kanal::Receiver<Vec<u8>>>,
    exit: Arc<AtomicBool>,
}
impl MQueue {
    pub fn new() -> Self {
        let (send, recv) = kanal::bounded(10_000);
        Self {
            send,
            subscriptions: Vec::new(),
            recv: Some(recv),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn link(&self) -> kanal::Sender<Vec<u8>> {
        self.send.clone()
    }
    pub fn subscribe(&mut self) -> kanal::Receiver<Vec<u8>> {
        let (send, recv) = kanal::bounded(10_000);
        self.subscriptions.push(send);
        recv
    }
    pub fn run(&mut self) -> JoinHandle<()> {
        let subscriptions: Vec<kanal::Sender<Vec<u8>>> = self.subscriptions.drain(0..).collect();
        let recv = self.recv.take().unwrap();
        let exit = self.exit.clone();
        thread::spawn(move|| {
            log::info!("MQueue.run | Start");
            loop {
                match recv.recv_timeout(Duration::from_millis(10)) {
                    Ok(event) => {
                        for send in &subscriptions {
                            if let Err(err) = send.send(event.clone()) {
                                log::warn!("MQueue.run | Send error: {:?}", err);
                            }
                        }
                    }
                    Err(err) => {
                        match err {
                            kanal::ReceiveErrorTimeout::Timeout => {}
                            kanal::ReceiveErrorTimeout::Closed => panic!("MQueue.run | Error: {:?}", err),
                            kanal::ReceiveErrorTimeout::SendClosed => panic!("MQueue.run | Error: {:?}", err),
                        }
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