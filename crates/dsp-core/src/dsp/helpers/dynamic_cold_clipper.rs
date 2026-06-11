use crate::dsp::{HighPassFilter, SampleProcessingNode};
use crate::utils::AtomicF32;
use std::sync::Arc;

pub struct DynamicColdClipper {
    static_bias: f32,
    dynamic_bias: f32,
    cathode_alpha: f32,
    dc_blocker: HighPassFilter,
}

impl DynamicColdClipper {
    pub fn new(sample_rate: f32, static_bias: f32) -> Self {
        let cathode_cutoff_hz = 20.0;
        let cathode_alpha =
            1.0 - (-2.0 * std::f32::consts::PI * cathode_cutoff_hz / sample_rate).exp();

        let dc_cutoff = Arc::new(AtomicF32::new(10.0));
        let dc_blocker = HighPassFilter::new(dc_cutoff, sample_rate);

        Self {
            static_bias, // smth like -0.45
            dynamic_bias: 0.0,
            cathode_alpha,
            dc_blocker,
        }
    }
}

impl SampleProcessingNode for DynamicColdClipper {
    fn process(&mut self, input: f32) -> f32 {
        let x = input;

        let total_bias = self.static_bias + self.dynamic_bias;
        let clipped = (x + total_bias).tanh();

        let signal_rectified = if clipped > 0.0 { clipped } else { 0.0 };
        let target_bias = -0.5 * signal_rectified;
        self.dynamic_bias += self.cathode_alpha * (target_bias - self.dynamic_bias);
        let zero_point = total_bias.tanh();
        let centered = clipped - zero_point;
        self.dc_blocker.process(centered)
    }
}
