use std::{f32::consts::PI, sync::Arc};

use crate::{dsp::filters::biquad::BiquadFilter, utils::AtomicF32};

pub struct HighShelfFilter {
    sample_rate: f32,
    filter: BiquadFilter,
    catoff_fr: Arc<AtomicF32>,
    gain_db: Arc<AtomicF32>,

    last_catoff_fr: f32,
    last_gain_db: f32,
}

impl HighShelfFilter {
    pub fn new(sample_rate: f32, catoff_fr: Arc<AtomicF32>, gain_db: Arc<AtomicF32>) -> Self {
        let mut hsf = Self {
            sample_rate,
            filter: BiquadFilter::new(1.0, 0.0, 0.0, 0.0, 0.0),
            last_catoff_fr: catoff_fr.get(),
            last_gain_db: gain_db.get(),
            catoff_fr,
            gain_db,
        };

        hsf.update_high_shelf();

        hsf
    }

    pub fn process(&mut self, sample: f32) -> f32 {
        self.update_if_need();
        self.filter.process(sample)
    }

    fn update_if_need(&mut self) {
        let catoff_changed = self.catoff_fr.get() != self.last_catoff_fr;
        let db_changed = self.gain_db.get() != self.last_gain_db;
        if catoff_changed || db_changed {
            self.update_high_shelf();
            self.last_catoff_fr = self.catoff_fr.get();
            self.last_gain_db = self.gain_db.get();
        }
    }

    fn update_high_shelf(&mut self) {
        let q = 0.707;
        let frequency = self.catoff_fr.get();
        let ac = 10.0_f32.powf(self.gain_db.get() / 40.0);

        let omega = 2.0 * PI * (frequency / self.sample_rate);
        let cos_w = omega.cos();
        let beta = 2.0 * ac.sqrt() * (omega.sin() / (2.0 * q));

        let b0 = ac * ((ac + 1.0) + (ac - 1.0) * cos_w + beta);
        let b1 = -2.0 * ac * ((ac - 1.0) + (ac + 1.0) * cos_w);
        let b2 = ac * ((ac + 1.0) + (ac - 1.0) * cos_w - beta);

        let a0 = (ac + 1.0) - (ac - 1.0) * cos_w + beta;
        let a1 = 2.0 * ((ac - 1.0) - (ac + 1.0) * cos_w);
        let a2 = (ac + 1.0) - (ac - 1.0) * cos_w - beta;

        self.filter
            .update_coefficients(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0);
    }
}

// TODO get rid of
pub struct HighShelfFilterSimpl {
    filter: BiquadFilter,
}

impl HighShelfFilterSimpl {
    pub fn new(sample_rate: f32, frequency: f32, gain_db: f32, q: f32) -> Self {
        let ac = 10.0_f32.powf(gain_db / 40.0);

        let omega = 2.0 * PI * (frequency / sample_rate);
        let cos_w = omega.cos();
        let beta = 2.0 * ac.sqrt() * (omega.sin() / (2.0 * q));

        let b0 = ac * ((ac + 1.0) + (ac - 1.0) * cos_w + beta);
        let b1 = -2.0 * ac * ((ac - 1.0) + (ac + 1.0) * cos_w);
        let b2 = ac * ((ac + 1.0) + (ac - 1.0) * cos_w - beta);

        let a0 = (ac + 1.0) - (ac - 1.0) * cos_w + beta;
        let a1 = 2.0 * ((ac - 1.0) - (ac + 1.0) * cos_w);
        let a2 = (ac + 1.0) - (ac - 1.0) * cos_w - beta;

        HighShelfFilterSimpl {
            filter: BiquadFilter::new(b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0),
        }
    }

    pub fn process(&mut self, x: f32) -> f32 {
        self.filter.process(x)
    }
}
