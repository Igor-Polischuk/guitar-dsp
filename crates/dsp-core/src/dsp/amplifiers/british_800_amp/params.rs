use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

use crate::{dsp::amplifiers::british_800_amp::knobs::BRITISH_800_KNOBS, utils::AtomicF32};

fn normalize_knob_linear(knob_value: f32) -> f32 {
    (knob_value / 10.0).clamp(0.0, 1.0)
}

fn normalize_knob_log(knob_value: f32) -> f32 {
    let normalized = normalize_knob_linear(knob_value);

    (1.0 + 9.0 * normalized).log10()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum British800Input {
    High,
    Low,
}

impl British800Input {
    fn as_u8(self) -> u8 {
        match self {
            British800Input::High => 0,
            British800Input::Low => 1,
        }
    }

    fn from_u8(value: u8) -> Self {
        match value {
            1 => British800Input::Low,
            _ => British800Input::High,
        }
    }

    fn from_id(id: &str) -> Option<Self> {
        match id {
            "high_input" => Some(British800Input::High),
            "low_input" => Some(British800Input::Low),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct British800Params {
    pub presence: Arc<AtomicF32>,
    pub bass: Arc<AtomicF32>,
    pub mid: Arc<AtomicF32>,
    pub treble: Arc<AtomicF32>,
    pub master_volume: Arc<AtomicF32>,
    pub pre_amp_volume: Arc<AtomicF32>,
    pub input: Arc<AtomicU8>,
}

impl British800Params {
    pub fn set(&self, id: &str, knob_value: f32) {
        match id {
            "pre_amp_volume" => {
                // Value from knob [1; 10] into actual data 0...1
                let normalized = (knob_value / 10.0).clamp(0.0, 1.0);
                let pot = normalized.powf(2.0);

                self.pre_amp_volume.set(pot);
            }
            "bass" => self.bass.set(normalize_knob_log(knob_value)),
            "mid" => self.mid.set(normalize_knob_linear(knob_value)),
            "treble" => self.treble.set(normalize_knob_linear(knob_value)),
            "presence" => {
                // let feedback_high_cut_db = -12.0 * knob_value;
                self.presence.set(knob_value)
            }
            "master_volume" => {
                let t = (knob_value / 10.0).clamp(0.0, 1.0);
                let volume;

                if t <= 0.0001 {
                    volume = -60.0; // mute
                } else {
                    volume = -40.0 + t.powf(0.7) * 40.0;
                }

                self.master_volume.set(volume)
            }
            _ => {}
        }
    }

    pub fn set_input(&self, id: &str) -> Result<(), String> {
        let input = British800Input::from_id(id)
            .ok_or_else(|| format!("Unknown British 800 input: {id}"))?;

        self.input.store(input.as_u8(), Ordering::Relaxed);
        Ok(())
    }

    pub fn active_input_id(&self) -> &'static str {
        match self.get_active_input() {
            British800Input::High => "high_input",
            British800Input::Low => "low_input",
        }
    }

    pub fn get_active_input(&self) -> British800Input {
        British800Input::from_u8(self.input.load(Ordering::Relaxed))
    }
}

impl Default for British800Params {
    fn default() -> Self {
        let params = British800Params {
            presence: Arc::new(AtomicF32::new(0.0)),
            bass: Arc::new(AtomicF32::new(0.0)),
            mid: Arc::new(AtomicF32::new(0.0)),
            treble: Arc::new(AtomicF32::new(0.0)),
            master_volume: Arc::new(AtomicF32::new(0.0)),
            pre_amp_volume: Arc::new(AtomicF32::new(0.25)),
            input: Arc::new(AtomicU8::new(British800Input::High.as_u8())),
        };

        // TODO: better handle default values (with enum)
        params.set("presence", BRITISH_800_KNOBS[0].default);
        params.set("bass", BRITISH_800_KNOBS[1].default);
        params.set("mid", BRITISH_800_KNOBS[2].default);
        params.set("treble", BRITISH_800_KNOBS[3].default);
        params.set("master_volume", BRITISH_800_KNOBS[4].default);
        params.set("pre_amp_volume", BRITISH_800_KNOBS[5].default);

        params
    }
}
