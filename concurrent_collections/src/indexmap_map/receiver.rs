use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use indexmap::IndexMap;
use crate::Event;

pub struct Receiver {
    index: usize,
    limit: usize,
    map: Arc<parking_lot::RwLock<IndexMap<String, Event>>>,
    exit: Arc<AtomicBool>,
}
impl Receiver {
    pub fn new(index: usize, limit: usize) -> Self {
        Self {
            index,
            limit,
            map: Arc::new(parking_lot::RwLock::new(IndexMap::new())),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn insert(&self, key: String, event: Event) {
        self.map.write().insert(key, event);
    }
    pub fn received(&self) -> usize {
        self.map.read().len()
    }
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}