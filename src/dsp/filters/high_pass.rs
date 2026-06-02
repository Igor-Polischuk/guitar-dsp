use std::f32;

use super::biquad::BiquadFilter;
use crate::dsp::AudioNode;

pub struct HighPassFilter {
    biquad_filter: BiquadFilter,
}

impl HighPassFilter {
    pub fn new(cutoff_hz: f32, sample_rate: f32) -> Self {
        let q = 0.707;

        let angular_freq = 2.0 * f32::consts::PI * (cutoff_hz / sample_rate);

        let alpha = angular_freq.sin() / (2.0 * q);

        let b0 = (1.0 + angular_freq.cos()) / 2.0;
        let b1 = -(1.0 + angular_freq.cos());
        let b2 = b0;

        let a0 = 1.0 + alpha;
        let a1 = -2.0 * angular_freq.cos();
        let a2 = 1.0 - alpha;

        let b0 = b0 / a0;
        let b1 = b1 / a0;
        let b2 = b2 / a0;
        let a1 = a1 / a0;
        let a2 = a2 / a0;

        let biquad_filter = BiquadFilter::new(b0, b1, b2, a1, a2);

        HighPassFilter { biquad_filter }
    }
}

impl AudioNode for HighPassFilter {
    fn process(&mut self, input: f32) -> f32 {
        self.biquad_filter.process(input)
    }
}

#[cfg(test)]
#[path = "high_pass_test.rs"]
mod high_pass_test;
