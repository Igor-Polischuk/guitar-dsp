pub struct RingBuffer<const N: usize, D> {
    buffer: [D; N],
    write_ptr: usize,
}

impl<const N: usize, D> RingBuffer<N, D>
where
    D: Default + Copy,
{
    pub fn new() -> Self {
        assert!(N > 0, "Buffer size must be greater than 0");
        Self {
            buffer: std::array::from_fn(|_| D::default()),
            write_ptr: 0,
        }
    }

    pub fn push(&mut self, sample: D) {
        self.buffer[self.write_ptr] = sample;

        self.write_ptr = (self.write_ptr + 1) % N;
    }

    pub fn get(&self, delay: usize) -> D {
        if delay >= N {
            return D::default();
        }

        let read_ptr = (self.write_ptr + N - 1 - delay) % N;

        self.buffer[read_ptr]
    }

    pub fn get_ref(&self, delay: usize) -> &D {
        if delay >= N {
            return &self.buffer[0];
        }

        let read_ptr = (self.write_ptr + N - 1 - delay) % N;

        &self.buffer[read_ptr]
    }
}
