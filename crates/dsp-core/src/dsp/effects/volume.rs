use crate::dsp::AudioNode;

pub struct MasterVolume {
    target_gain: f32,
    current_gain: f32,
    smoothing_factor: f32,
}

impl MasterVolume {
    pub fn new(sample_rate: f32) -> Self {
        let time_constant_ms = 30.0;
        let smoothing_factor = 1.0 - (-1.0 / (sample_rate * (time_constant_ms / 1000.0))).exp();

        let mut volume = Self {
            target_gain: 0.0,
            current_gain: 0.0,
            smoothing_factor,
        };

        volume.set_knob(10);

        volume
    }

    pub fn set_knob(&mut self, knob_value: u8) {
        let knob = knob_value.min(10) as f32;

        if knob == 0.0 {
            self.target_gain = 0.0;
        } else {
            let db = -60.0 + knob * 6.0; // 1=-54dB, 10=0dB
            self.target_gain = 10.0_f32.powf(db / 20.0);
        }
    }
}

impl AudioNode for MasterVolume {
    fn process(&mut self, input: f32) -> f32 {
        self.current_gain += (self.target_gain - self.current_gain) * self.smoothing_factor;
        input * self.current_gain
    }
}
