use std::sync::atomic::{AtomicU32, Ordering};

pub struct Counter(AtomicU32);

impl Counter {
    pub fn increment(&self) -> u32 {
        self.0.fetch_add(1, Ordering::Relaxed);
        self.0.load(Ordering::Relaxed)
    }
}

pub fn create_counter() -> Counter {
    Counter(AtomicU32::new(0))
}
