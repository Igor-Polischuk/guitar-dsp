pub struct InputBlockAssembler<const N: usize> {
    block: [f32; N],
    index: usize,
}

impl<const N: usize> InputBlockAssembler<N> {
    pub fn new() -> Self {
        Self {
            block: [0.0; N],
            index: 0,
        }
    }

    pub fn push_sample(&mut self, sample: f32) -> Option<[f32; N]> {
        self.block[self.index] = sample;
        self.index += 1;

        if self.index == N {
            let ready = self.block;
            self.block = [0.0; N];
            self.index = 0;
            Some(ready)
        } else {
            None
        }
    }

    pub fn pending_samples(&self) -> usize {
        self.index
    }
}
