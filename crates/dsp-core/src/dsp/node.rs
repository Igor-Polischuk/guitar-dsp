pub trait SampleProcessingNode {
    fn process(&mut self, input: f32) -> f32;
}

pub trait BlockProcessingNode {
    fn process_block(&mut self, samples: &mut [f32]);
}
