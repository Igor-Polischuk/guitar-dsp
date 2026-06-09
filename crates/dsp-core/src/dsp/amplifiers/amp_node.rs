use crate::dsp::{SampleProcessingNode, amplifiers::amp_param::KnobDescriptor};

pub trait AmpNode: SampleProcessingNode + Send {
    fn model_id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn knobs(&self) -> &'static [KnobDescriptor];
}
