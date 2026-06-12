pub mod dsp;
pub mod utils;
pub mod wdf;

pub mod prelude {
    pub use crate::dsp::{
        British800Amp, British800Params, Cabinet, CabinetFactory, Distortion, DistortionPreset,
        Equalizer, EqualizerParams, Gain, HighPassFilter, LowPassFilter, MasterVolume,
        SampleProcessingChain, SampleProcessingNode, SignalChain, detect_pitch, hz_to_note,
    };

    pub use crate::utils::AtomicF32;
}
