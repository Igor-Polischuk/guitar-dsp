use crate::dsp::{
    SampleProcessingNode,
    amplifiers::{
        amp_node::AmpNode,
        amp_param::KnobDescriptor,
        british_800_amp::{knobs::BRITISH_800_KNOBS, params::British800Params},
    },
};

/// Insparation is Marshall Lead JCM800
pub struct British800Amp {
    params: British800Params,
}

impl SampleProcessingNode for British800Amp {
    fn process(&mut self, input: f32) -> f32 {
        input
    }
}

impl AmpNode for British800Amp {
    fn knobs(&self) -> &'static [KnobDescriptor] {
        BRITISH_800_KNOBS
    }

    fn model_id(&self) -> &'static str {
        "british_800"
    }

    fn name(&self) -> &'static str {
        "British 800"
    }
}

impl British800Amp {
    pub fn new() -> Self {
        British800Amp {
            params: British800Params::default(),
        }
    }
}
