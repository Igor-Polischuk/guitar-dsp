use ringbuf::traits::Consumer;

pub struct OutputBlockReader<const N: usize> {
    block: [f32; N],
    index: usize,
    has_block: bool,
}

impl<const N: usize> OutputBlockReader<N> {
    pub fn new() -> Self {
        Self {
            block: [0.0; N],
            has_block: false,
            index: 0,
        }
    }

    pub fn load_block(&mut self, block: [f32; N]) {
        self.block = block;
        self.index = 0;
        self.has_block = true;
    }

    pub fn pop_sample(&mut self) -> Option<f32> {
        if !self.has_block {
            return None;
        }

        let sample = self.block[self.index];
        self.index += 1;

        if self.index == N {
            self.index = 0;
            self.has_block = false;
        }

        Some(sample)
    }

    pub fn remaining_samples(&self) -> usize {
        N - self.index
    }
}

pub fn next_output_sample<const N: usize, C>(
    reader: &mut OutputBlockReader<N>,
    blocks: &mut C,
) -> Option<f32>
where
    C: Consumer<Item = [f32; N]>,
{
    if let Some(sample) = reader.pop_sample() {
        return Some(sample);
    }

    if let Some(block) = blocks.try_pop() {
        reader.load_block(block);
        return reader.pop_sample();
    }

    None
}
