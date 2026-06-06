pub trait AudioNode {
    fn process(&mut self, input: f32) -> f32;
    // fn set_sample_rate(&mut self, sample_rate: f32);
}
