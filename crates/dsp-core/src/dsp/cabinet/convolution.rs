use crate::dsp::{AudioNode, helpers::ring_buffer::RingBuffer};

pub struct Convolution<const N: usize> {
    ir: [f32; N],
    history: RingBuffer<N>,
}

impl<const N: usize> Convolution<N> {
    pub fn new(ir: [f32; N]) -> Self {
        Convolution {
            history: RingBuffer::new(),
            ir,
        }
    }
}

impl<const N: usize> AudioNode for Convolution<N> {
    fn process(&mut self, input: f32) -> f32 {
        let mut output = 0.0;
        self.history.push(input);

        for index in 0..self.ir.len() {
            output += self.ir[index] * self.history.get(index);
        }

        output
    }
}
