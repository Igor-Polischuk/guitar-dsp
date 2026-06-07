use std::sync::Arc;

use crate::{dsp::AudioNode, utils::AtomicF32};

pub struct MasterVolume {
    target_gain: f32,
    current_gain: f32,
    last_volume_db: f32,
    smoothing_factor: f32,
    volume_db: Arc<AtomicF32>,
}
impl MasterVolume {
    pub fn new(sample_rate: f32, volume_db: Arc<AtomicF32>) -> Self {
        let time_constant_ms = 30.0;
        let smoothing_factor = 1.0 - (-1.0 / (sample_rate * (time_constant_ms / 1000.0))).exp();

        let db = volume_db.get();
        let gain = db_to_gain(db);

        Self {
            target_gain: gain,
            current_gain: gain,
            last_volume_db: db,
            smoothing_factor,
            volume_db,
        }
    }
}

impl AudioNode for MasterVolume {
    #[inline]
    fn process(&mut self, input: f32) -> f32 {
        let db = self.volume_db.get();

        if db != self.last_volume_db {
            self.last_volume_db = db;
            self.target_gain = db_to_gain(db);
        }

        self.current_gain += (self.target_gain - self.current_gain) * self.smoothing_factor;

        input * self.current_gain
    }
}

fn db_to_gain(db: f32) -> f32 {
    if db <= -60.0 {
        0.0
    } else {
        10.0_f32.powf(db / 20.0)
    }
}
