use crate::dsp::{
    SampleProcessingNode,
    amplifiers::amp_param::{InputDescriptor, KnobDescriptor},
};

pub trait AmpNode: SampleProcessingNode + Send + Sync {
    fn model_id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn knobs() -> &'static [KnobDescriptor];
    fn inputs() -> &'static [InputDescriptor];
}
