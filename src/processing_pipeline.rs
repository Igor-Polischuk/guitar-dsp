pub struct ProcessingPipeline {
    input: f32,
    signal: f32,
    gain_clamp: (f32, f32),
}

impl ProcessingPipeline {
    pub fn new(input: f32) -> Self {
        ProcessingPipeline {
            input: input,
            signal: input,
            gain_clamp: (-0.95, 0.95),
        }
    }

    pub fn apply_gain(&mut self, value: f32) -> &mut Self {
        self.signal = (self.signal * value).clamp(self.gain_clamp.0, self.gain_clamp.1);
        self
    }

    pub fn apply_distortion(&mut self) -> &mut Self {
        // let positive_gain = 2.1;
        // let negative_gain = 0.45;
        // let tanh_gain = 1.3;
        // light lamp
        // let positive_gain = 1.5;
        // let negative_gain = 0.9;
        // let tanh_gain = 1.2;
        // crunch
        let positive_gain = 3.1;
        let negative_gain = 0.75;
        let tanh_gain = 1.5;
        // transistor
        // let positive_gain = 2.;
        // let negative_gain = 1.;
        // let tanh_gain = 2.;

        if self.signal < 0.0 {
            self.signal = negative_gain * (tanh_gain * self.signal).tanh()
        } else {
            self.signal = (positive_gain * self.signal).tanh()
        }

        self
    }

    pub fn get_output(&self) -> f32 {
        self.signal
    }
}
