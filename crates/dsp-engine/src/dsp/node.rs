pub trait AudioNode {
    fn process(&mut self, input: f32) -> f32;
}
