use super::{BlockProcessingNode, SampleProcessingNode};

pub struct SignalChain {
    nodes: Vec<Box<dyn BlockProcessingNode + Send>>,
}

impl SignalChain {
    pub fn new() -> Self {
        SignalChain { nodes: vec![] }
    }

    pub fn append_node<T: BlockProcessingNode + Send + 'static>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }

    pub fn process(&mut self, samples_block: &mut [f32]) {
        if !self.nodes.is_empty() {
            for node in &mut self.nodes {
                node.process_block(samples_block);
            }
        }
    }
}

pub struct SampleProcessingChain {
    nodes: Vec<Box<dyn SampleProcessingNode + Send>>,
}

impl SampleProcessingChain {
    pub fn new() -> Self {
        SampleProcessingChain { nodes: vec![] }
    }

    pub fn append_node<T: SampleProcessingNode + Send + 'static>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }
}

impl BlockProcessingNode for SampleProcessingChain {
    fn process_block(&mut self, samples: &mut [f32]) {
        if self.nodes.is_empty() {
            return;
        }

        for sample in samples.iter_mut() {
            let mut processed = *sample;

            for node in &mut self.nodes {
                processed = node.process(processed);
            }

            *sample = processed;
        }
    }
}
