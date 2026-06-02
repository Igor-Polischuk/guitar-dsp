pub mod dsp;

pub mod prelude {
    pub use crate::dsp::{
        AudioNode, Distortion, DistortionPreset, Gain, HighPassFilter, LowPassFilter, SignalChain,
        detect_pitch, hz_to_note,
    };
}
