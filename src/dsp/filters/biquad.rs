#[derive(Debug)]
pub struct BiquadFilter {
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,

    b0: f32,
    b1: f32,
    b2: f32,

    a1: f32,
    a2: f32,
}

impl BiquadFilter {
    pub fn new(b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) -> Self {
        BiquadFilter {
            a1,
            a2,
            b0,
            b1,
            b2,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let output = self.b0 * input + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = input;

        self.y2 = self.y1;
        self.y1 = output;

        output
    }
}
