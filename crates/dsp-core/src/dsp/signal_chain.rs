use super::AudioNode;

pub struct SignalChain {
    nodes: Vec<Box<dyn AudioNode + Send>>,
    sample_rate: f32,
}

impl SignalChain {
    pub fn new(sample_rate: f32) -> Self {
        SignalChain {
            nodes: vec![],
            sample_rate,
        }
    }

    pub fn append_node<T: AudioNode + Send + 'static>(&mut self, mut node: T) {
        // node.set_sample_rate(self.sample_rate);
        self.nodes.push(Box::new(node));
    }

    pub fn process(&mut self, signal_value: f32) -> f32 {
        let mut processed = signal_value;
        for node in &mut self.nodes {
            processed = node.process(processed);
        }

        processed
    }
}
