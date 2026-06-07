use std::sync::atomic::{AtomicU32, Ordering};

pub struct AtomicF32 {
    value: AtomicU32,
}

impl AtomicF32 {
    pub fn new(v: f32) -> Self {
        Self {
            value: AtomicU32::new(v.to_bits()),
        }
    }

    pub fn get(&self) -> f32 {
        f32::from_bits(self.value.load(Ordering::Relaxed))
    }

    pub fn set(&self, v: f32) {
        self.value.store(v.to_bits(), Ordering::Relaxed);
    }
}
