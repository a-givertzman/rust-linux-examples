use std::sync::{atomic::{AtomicBool, Ordering}, mpsc, Arc};
use tokio::task::JoinHandle;
use crate::event::Event;
///
/// 
pub struct Producer {
    index: usize,
    send: Option<mpsc::Sender<Event>>,
    data: Vec<Event>,
    exit: Arc<AtomicBool>,
}
//
//
impl Producer {
    ///
    /// 
    pub fn new(index: usize, send: mpsc::Sender<Event>, data: &[Event]) -> Self {
        Self {
            index,
            send: Some(send),
            data: data.into(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    ///
    /// 
    pub fn run(&mut self) -> JoinHandle<()> {
        let index = self.index;
        let data: Vec<Event> = self.data.drain(0..).collect();
        let send = self.send.take().unwrap();
        let exit = self.exit.clone();
        tokio::task::spawn_blocking(move|| {
            log::info!("Producer({}).run | Start", index);
            for event in data {
                match send.send(event) {
                    Ok(_) => {}
                    Err(err) => {
                        log::info!("Producer({}).run | Error: {:?}", index, err);
                    }
                }
                if exit.load(Ordering::SeqCst) {
                    break;
                }
            }
            log::info!("Producer({}).run | Exit", index);
        })
    }
    ///
    /// 
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}