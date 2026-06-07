use std::sync::Arc;

use crate::{dsp::AudioNode, utils::AtomicF32};

pub struct Gain {
    value: Arc<AtomicF32>,
}

impl Gain {
    pub fn new(value: Arc<AtomicF32>) -> Self {
        Gain { value }
    }
}

impl AudioNode for Gain {
    fn process(&mut self, input: f32) -> f32 {
        input * self.value.get()
    }
}
