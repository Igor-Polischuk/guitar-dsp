pub struct RingBuffer<const N: usize> {
    buffer: [f32; N],
    write_ptr: usize,
}

impl<const N: usize> RingBuffer<N> {
    pub fn new() -> Self {
        assert!(N > 0, "Buffer size must be greater than 0");
        Self {
            buffer: [0.0; N],
            write_ptr: 0,
        }
    }

    pub fn push(&mut self, sample: f32) {
        self.buffer[self.write_ptr] = sample;

        self.write_ptr = (self.write_ptr + 1) % N;
    }

    pub fn get(&self, delay: usize) -> f32 {
        if delay >= N {
            return 0.0;
        }

        let read_ptr = (self.write_ptr + N - 1 - delay) % N;

        self.buffer[read_ptr]
    }
}
