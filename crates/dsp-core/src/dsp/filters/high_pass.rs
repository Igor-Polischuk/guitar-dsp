use std::{f32, sync::Arc};

use super::biquad::BiquadFilter;
use crate::{dsp::SampleProcessingNode, utils::AtomicF32};

pub struct HighPassFilter {
    biquad_filter: BiquadFilter,
    last_cutoff_value: f32,
    cutoff_hz: Arc<AtomicF32>,
    sample_rate: f32,
}

impl HighPassFilter {
    pub fn new(cutoff_hz: Arc<AtomicF32>, sample_rate: f32) -> Self {
        let cutoff = cutoff_hz.get();

        Self {
            biquad_filter: Self::make_biquad(cutoff, sample_rate),
            last_cutoff_value: cutoff,
            cutoff_hz,
            sample_rate,
        }
    }

    fn make_biquad(cutoff_hz: f32, sample_rate: f32) -> BiquadFilter {
        let q = 0.707;

        let omega = 2.0 * f32::consts::PI * (cutoff_hz / sample_rate);

        let sin_w = omega.sin();
        let cos_w = omega.cos();
        let alpha = sin_w / (2.0 * q);

        let b0 = (1.0 + cos_w) / 2.0;
        let b1 = -(1.0 + cos_w);
        let b2 = b0;

        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w;
        let a2 = 1.0 - alpha;

        BiquadFilter::new(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }

    fn update_if_needed(&mut self) {
        let cutoff = self.cutoff_hz.get();

        if cutoff != self.last_cutoff_value {
            self.biquad_filter = Self::make_biquad(cutoff, self.sample_rate);
            self.last_cutoff_value = cutoff;
        }
    }
}

impl SampleProcessingNode for HighPassFilter {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        self.update_if_needed();
        self.biquad_filter.process(input)
    }
}

#[cfg(test)]
#[path = "high_pass_test.rs"]
mod high_pass_test;
