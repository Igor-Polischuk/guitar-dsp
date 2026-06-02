use super::AudioNode;

pub struct SignalChain {
    nodes: Vec<Box<dyn AudioNode + Send>>,
}

impl SignalChain {
    pub fn new() -> Self {
        SignalChain { nodes: vec![] }
    }

    pub fn append_node<T: AudioNode + Send + 'static>(&mut self, node: T) {
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

impl Default for SignalChain {
    fn default() -> Self {
        Self::new()
    }
}
