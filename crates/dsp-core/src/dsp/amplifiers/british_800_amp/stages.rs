use std::sync::Arc;

use crate::{
    dsp::{HighPassFilter, LowPassFilter, helpers::dynamic_cold_clipper::DynamicColdClipper},
    utils::AtomicF32,
};

pub struct InputStage {
    pub hpf_low_input: HighPassFilter,
    pub hpf_high_input: HighPassFilter,
}

impl InputStage {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            hpf_low_input: HighPassFilter::new(Arc::new(AtomicF32::new(100.0)), sample_rate),
            hpf_high_input: HighPassFilter::new(Arc::new(AtomicF32::new(70.0)), sample_rate),
        }
    }
}

pub struct GainStage {
    pub hpf_1: HighPassFilter,
    pub lpf_1: LowPassFilter,
    pub hpf_2: HighPassFilter,
    pub lpf_2: LowPassFilter,
    pub hpf_3: HighPassFilter,
    pub lpf_3: LowPassFilter,
    pub cold_clipper: DynamicColdClipper,
}

impl GainStage {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            hpf_1: HighPassFilter::new(Arc::new(AtomicF32::new(400.0)), sample_rate),
            lpf_1: LowPassFilter::new(Arc::new(AtomicF32::new(8000.0)), sample_rate),
            hpf_2: HighPassFilter::new(Arc::new(AtomicF32::new(150.0)), sample_rate),
            lpf_2: LowPassFilter::new(Arc::new(AtomicF32::new(6000.0)), sample_rate),
            hpf_3: HighPassFilter::new(Arc::new(AtomicF32::new(100.0)), sample_rate),
            lpf_3: LowPassFilter::new(Arc::new(AtomicF32::new(7000.0)), sample_rate),
            cold_clipper: DynamicColdClipper::new(sample_rate, -0.45),
        }
    }
}
