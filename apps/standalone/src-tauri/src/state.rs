use std::sync::Mutex;

use audio_io::prelude::AudioIO;
use dsp_core::dsp::{ActiveAmpParams, AmpModel};

pub struct ActiveAmpState {
    pub model: AmpModel,
    pub params: ActiveAmpParams,
}

pub struct EngineState {
    pub audio: Mutex<AudioIO>,
    pub active_amp: Mutex<ActiveAmpState>,
}
