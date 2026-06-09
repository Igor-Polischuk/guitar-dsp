pub mod amplifiers;
pub mod cabinet;
pub mod effects;
pub mod filters;

mod helpers;
mod node;
mod pitch;
mod signal_chain;

pub use amplifiers::{British800Amp, British800Params};
pub use cabinet::cabinet::{Cabinet, CabinetFactory};
pub use effects::{Distortion, DistortionPreset, Gain, MasterVolume};
pub use filters::{Equalizer, EqualizerParams, HighPassFilter, LowPassFilter};
pub use node::{BlockProcessingNode, SampleProcessingNode};
pub use pitch::{detect_pitch, hz_to_note};
pub use signal_chain::{SampleProcessingChain, SignalChain};
