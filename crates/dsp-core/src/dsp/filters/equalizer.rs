use std::{
    f32::consts::PI,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::{
    dsp::{SampleProcessingNode, filters::biquad::BiquadFilter},
    utils::AtomicF32,
};

pub struct EqualizerParams {
    pub treble: AtomicF32,
    pub mid: AtomicF32,
    pub bass: AtomicF32,

    pub treble_version: AtomicU64,
    pub mid_version: AtomicU64,
    pub bass_version: AtomicU64,
}

impl EqualizerParams {
    pub fn set_bass(&self, value: f32) {
        self.bass.set(value);
        self.bass_version.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_mid(&self, value: f32) {
        self.mid.set(value);
        self.mid_version.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_treble(&self, value: f32) {
        self.treble.set(value);
        self.treble_version.fetch_add(1, Ordering::Relaxed);
    }
}

impl Default for EqualizerParams {
    fn default() -> Self {
        EqualizerParams {
            bass: AtomicF32::new(0.0),
            mid: AtomicF32::new(0.0),
            treble: AtomicF32::new(0.0),
            bass_version: AtomicU64::new(0),
            mid_version: AtomicU64::new(0),
            treble_version: AtomicU64::new(0),
        }
    }
}

pub struct Equalizer {
    params: Arc<EqualizerParams>,

    last_treble_version: u64,
    last_mid_version: u64,
    last_bass_version: u64,

    sample_rate: f32,
    peaking_eq: BiquadFilter, // Mid range
    low_shelf: BiquadFilter,  // Mid range
    high_shelf: BiquadFilter, // Treble range
}

impl Equalizer {
    pub fn new(sample_rate: f32, params: Arc<EqualizerParams>) -> Self {
        let mut eq = Equalizer {
            sample_rate,
            params,
            last_bass_version: 0,
            last_mid_version: 0,
            last_treble_version: 0,
            peaking_eq: BiquadFilter::new(1.0, 0.0, 0.0, 0.0, 0.0),
            low_shelf: BiquadFilter::new(1.0, 0.0, 0.0, 0.0, 0.0),
            high_shelf: BiquadFilter::new(1.0, 0.0, 0.0, 0.0, 0.0),
        };

        eq.update_peaking_eq();
        eq.update_low_shelf();
        eq.update_high_shelf();
        eq
    }

    fn update_peaking_eq(&mut self) {
        let q = 1.0;
        let frequency = 800.0;
        let amplitude_coef = 10.0_f32.powf(self.params.mid.get() / 40.0);

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
        let ac = 10.0_f32.powf(self.params.bass.get() / 40.0);

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
        let ac = 10.0_f32.powf(self.params.treble.get() / 40.0);

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

    fn update_if_needed(&mut self) {
        let bass_version = self.params.bass_version.load(Ordering::Relaxed);
        if self.last_bass_version != bass_version {
            self.last_bass_version = bass_version;
            self.update_low_shelf();
        }

        let mid_version = self.params.mid_version.load(Ordering::Relaxed);
        if self.last_mid_version != mid_version {
            self.last_mid_version = mid_version;
            self.update_peaking_eq();
        }

        let treble_version = self.params.treble_version.load(Ordering::Relaxed);
        if self.last_treble_version != treble_version {
            self.last_treble_version = treble_version;
            self.update_high_shelf();
        }
    }
}

impl SampleProcessingNode for Equalizer {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        self.update_if_needed();
        let out = self.low_shelf.process(input);
        let out = self.peaking_eq.process(out);
        self.high_shelf.process(out)
    }
}
// DB range must be in [-12; 12]
fn knob_to_db(knob: u8) -> f32 {
    ((knob.min(10) as f32) - 5.0) * 2.4
}
