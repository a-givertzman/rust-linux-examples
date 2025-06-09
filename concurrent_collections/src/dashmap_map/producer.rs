use std::{sync::{atomic::{AtomicBool, Ordering}, Arc}, thread::{self, JoinHandle}};
use crate::Event;
use super::Receiver;

pub struct Producer {
    index: usize,
    service: Arc<Receiver>,
    data: Vec<Event>,
    exit: Arc<AtomicBool>,
}
impl Producer {
    pub fn new(index: usize, service: Arc<Receiver>, data: &[Event]) -> Self {
        Self {
            index,
            service,
            data: data.into(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn run(&mut self) -> JoinHandle<()> {
        let index = self.index;
        let data: Vec<Event> = self.data.drain(0..).collect();
        let service = self.service.clone();
        let exit = self.exit.clone();
        thread::spawn(move|| {
            log::debug!("Producer({}).run | Start", index);
            for (key, event) in data.into_iter().enumerate() {
                service.insert(format!("{index}.{key}"), event);
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::debug!("Producer({}).run | Exit", index);
        })
    }
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}
