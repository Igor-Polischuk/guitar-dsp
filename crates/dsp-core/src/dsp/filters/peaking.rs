use std::f32::consts::PI;

use crate::dsp::filters::biquad::BiquadFilter;

pub struct PeakingFilter {
    filter: BiquadFilter,
}

impl PeakingFilter {
    pub fn new(sample_rate: f32, frequency: f32, gain_db: f32, q: f32) -> Self {
        let ac = 10.0_f32.powf(gain_db / 40.0);
        let omega = 2.0 * PI * (frequency / sample_rate);
        let alpha = omega.sin() / (2.0 * q);

        let b0 = 1.0 + alpha * ac;
        let b1 = -2.0 * omega.cos();
        let b2 = 1.0 - alpha * ac;

        let a0 = 1.0 + alpha / ac;
        let a1 = -2.0 * omega.cos();
        let a2 = 1.0 - alpha / ac;

        Self {
            filter: BiquadFilter::new(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0),
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        self.filter.process(input)
    }
}
