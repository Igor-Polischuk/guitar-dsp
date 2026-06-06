pub mod dsp;

pub mod prelude {
    pub use crate::dsp::{
        AudioNode, Cabinet, CabinetManager, Distortion, DistortionPreset, Equalizer, Gain,
        HighPassFilter, LowPassFilter, MasterVolume, SignalChain, detect_pitch, hz_to_note,
    };
}
