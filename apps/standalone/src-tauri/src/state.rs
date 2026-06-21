use std::sync::{Arc, Mutex};

use audio_io::prelude::AudioIO;
use dsp_core::{
    dsp::{ActiveAmpParams, AmpModel},
    utils::AtomicF32,
};

pub struct AudioMeters {
    pub input_peak_dbfs: Arc<AtomicF32>,
    pub output_peak_dbfs: Arc<AtomicF32>,
}

pub struct ActiveAmpState {
    pub model: AmpModel,
    pub params: ActiveAmpParams,
}

pub struct EngineState {
    pub audio: Mutex<AudioIO>,
    pub active_amp: Mutex<ActiveAmpState>,
    pub audio_meters: AudioMeters,
}
