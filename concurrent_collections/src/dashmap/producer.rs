use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, JoinHandle}};

use crate::event::Event;

pub struct Producer {
    index: usize,
    send: Option<kanal::Sender<Vec<u8>>>,
    data: Vec<Event>,
    exit: Arc<AtomicBool>,
}
impl Producer {
    pub fn new(index: usize, send: kanal::Sender<Vec<u8>>, data: &[Event]) -> Self {
        Self {
            index,
            send: Some(send),
            data: data.into(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn run(&mut self) -> JoinHandle<()> {
        let index = self.index;
        let data: Vec<Event> = self.data.drain(0..).collect();
        let send = self.send.take().unwrap();
        let exit = self.exit.clone();
        thread::spawn(move|| {
            log::info!("Producer({}).run | Start", index);
            let config = bincode::config::standard();
            for event in data {
                match bincode::encode_to_vec(event, config) {
                    Ok(event) => match send.send(event) {
                        Ok(_) => {}
                        Err(err) => {
                            log::error!("Producer({}).run | Error: {:?}", index, err);
                        }
                    }
                    Err(err) => log::error!("Producer({}).run | Error: {:?}", index, err),
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("Producer({}).run | Exit", index);
        })
    }
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
