use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use crossbeam_skiplist::SkipMap;
use crate::Event;

pub struct Receiver {
    index: usize,
    limit: usize,
    map: SkipMap<String, Event>,
    exit: Arc<AtomicBool>,
}
impl Receiver {
    pub fn new(index: usize, limit: usize) -> Self {
        Self {
            index,
            limit,
            map: SkipMap::new(),
            exit: Arc::new(AtomicBool::new(false)),
        }
    }
    pub fn insert(&self, key: String, event: Event) {
        // self.map.insert(key, event);
    }
    pub fn received(&self) -> usize {
        // self.map.len()
        0
    }
    pub fn exit(&self) {
        self.exit.store(true, Ordering::SeqCst);
    }
}