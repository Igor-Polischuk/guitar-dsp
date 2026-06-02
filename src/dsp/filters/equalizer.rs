use std::f32::consts::PI;

use crate::dsp::{AudioNode, filters::biquad::BiquadFilter};

pub struct Equalizer {
    treble_knob: u8,
    mid_knob: u8,
    bass_knob: u8,

    sample_rate: f32,
    peaking_eq: BiquadFilter, // Mid range
    low_shelf: BiquadFilter,  // Mid range
    high_shelf: BiquadFilter, // Treble range
}

impl Equalizer {
    pub fn new(sample_rate: f32) -> Self {
        let mut eq = Equalizer {
            sample_rate,
            bass_knob: 5,
            mid_knob: 5,
            treble_knob: 5,
            peaking_eq: BiquadFilter::new(1.0, 0.0, 0.0, 0.0, 0.0),
            low_shelf: BiquadFilter::new(1.0, 0.0, 0.0, 0.0, 0.0),
            high_shelf: BiquadFilter::new(1.0, 0.0, 0.0, 0.0, 0.0),
        };

        eq.update_peaking_eq();
        eq.update_low_shelf();
        eq.update_high_shelf();
        eq
    }

    pub fn set_mid_knob(&mut self, knob_value: u8) {
        self.mid_knob = knob_value.min(10); // Безпечне затискання 0..10
        self.update_peaking_eq();
    }

    pub fn set_bass_knob(&mut self, knob_value: u8) {
        self.bass_knob = knob_value.min(10);
        self.update_low_shelf();
    }

    pub fn set_treble_knob(&mut self, knob_value: u8) {
        self.treble_knob = knob_value.min(10);
        self.update_high_shelf();
    }

    fn update_peaking_eq(&mut self) {
        let q = 1.0;
        let frequency = 800.0;
        let amplitude_coef = 10.0_f32.powf(knob_to_db(self.mid_knob) / 40.0);

        let omega = 2.0 * PI * (frequency / self.sample_rate);
        let alpha = omega.sin() / (2.0 * q);

        let b0 = 1.0 + alpha * amplitude_coef;
        let b1 = -2.0 * omega.cos();
        let b2 = 1.0 - alpha * amplitude_coef;

        let a0 = 1.0 + alpha / amplitude_coef;
        let a1 = -2.0 * omega.cos();
        let a2 = 1.0 - alpha / amplitude_coef;

        self.peaking_eq
            .update_coefficients(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0);
    }

    fn update_low_shelf(&mut self) {
        let q = 0.707;
        let frequency = 120.0;
        let ac = 10.0_f32.powf(knob_to_db(self.bass_knob) / 40.0);

        let omega = 2.0 * PI * (frequency / self.sample_rate);
        let cos_w = omega.cos();
        let beta = 2.0 * ac.sqrt() * (omega.sin() / (2.0 * q));

        let b0 = ac * ((ac + 1.0) - (ac - 1.0) * cos_w + beta);
        let b1 = 2.0 * ac * ((ac - 1.0) - (ac + 1.0) * cos_w);
        let b2 = ac * ((ac + 1.0) - (ac - 1.0) * cos_w - beta);

        let a0 = (ac + 1.0) + (ac - 1.0) * cos_w + beta;
        let a1 = -2.0 * ((ac - 1.0) + (ac + 1.0) * cos_w);
        let a2 = (ac + 1.0) + (ac - 1.0) * cos_w - beta;

        self.low_shelf
            .update_coefficients(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0);
    }

    fn update_high_shelf(&mut self) {
        let q = 0.707;
        let frequency = 3500.0;
        let ac = 10.0_f32.powf(knob_to_db(self.treble_knob) / 40.0);

        let omega = 2.0 * PI * (frequency / self.sample_rate);
        let cos_w = omega.cos();
        let beta = 2.0 * ac.sqrt() * (omega.sin() / (2.0 * q));

        let b0 = ac * ((ac + 1.0) + (ac - 1.0) * cos_w + beta);
        let b1 = -2.0 * ac * ((ac - 1.0) + (ac + 1.0) * cos_w);
        let b2 = ac * ((ac + 1.0) + (ac - 1.0) * cos_w - beta);

        let a0 = (ac + 1.0) - (ac - 1.0) * cos_w + beta;
        let a1 = 2.0 * ((ac - 1.0) - (ac + 1.0) * cos_w);
        let a2 = (ac + 1.0) - (ac - 1.0) * cos_w - beta;

        self.high_shelf
            .update_coefficients(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0);
    }
}

impl AudioNode for Equalizer {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let out = self.low_shelf.process(input);
        let out = self.peaking_eq.process(out);
        self.high_shelf.process(out)
    }
}
// DB range must be in [-12; 12]
fn knob_to_db(knob: u8) -> f32 {
    ((knob.min(10) as f32) - 5.0) * 2.4
}
