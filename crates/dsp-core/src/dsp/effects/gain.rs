use std::sync::Arc;

use crate::{dsp::SampleProcessingNode, utils::AtomicF32};

pub struct Gain {
    value: Arc<AtomicF32>,
}

impl Gain {
    pub fn new(value: Arc<AtomicF32>) -> Self {
        Gain { value }
    }
}

impl SampleProcessingNode for Gain {
    fn process(&mut self, input: f32) -> f32 {
        input * self.value.get()
    }
}
