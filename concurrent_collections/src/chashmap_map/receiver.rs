use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use crate::Event;

pub struct Receiver {
    index: usize,
    limit: usize,
    map: chashmap::CHashMap<String, Event>,
    exit: Arc<AtomicBool>,
}
impl Receiver {
    pub fn new(index: usize, limit: usize) -> Self {
        Self {
            index,
            limit,
            map: chashmap::CHashMap::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn insert(&self, key: String, event: Event) {
        self.map.insert(key, event);
    }
    pub fn received(&self) -> usize {
        self.map.len()
    }
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}