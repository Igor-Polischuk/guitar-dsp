use std::{num::FpCategory::Nan, sync::Arc};

use crate::{
    dsp::{
        HighPassFilter, LowPassFilter, filters::HighShelfFilter,
        helpers::dynamic_cold_clipper::DynamicColdClipper,
    },
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

pub struct PowerAmpStage {
    pub hpf: HighPassFilter,
    pub lpf: LowPassFilter,
    pub last_power_amp_output: f32,
    pub presence_hpf: HighPassFilter,
}

impl PowerAmpStage {
    pub fn new(sample_rate: f32, presence: Arc<AtomicF32>) -> Self {
        Self {
            hpf: HighPassFilter::new(Arc::new(AtomicF32::new(50.0)), sample_rate),
            presence_hpf: HighPassFilter::new(Arc::new(AtomicF32::new(1000.0)), sample_rate),
            lpf: LowPassFilter::new(Arc::new(AtomicF32::new(10000.0)), sample_rate),
            last_power_amp_output: 0.0,
            // presence_feedback_filter: HighShelfFilter::new(
            //     sample_rate,
            //     Arc::new(AtomicF32::new(1000.0)),
            //     presence,
            // ),
        }
    }
}
