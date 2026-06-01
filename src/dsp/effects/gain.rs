use crate::dsp::AudioNode;

pub struct Gain {
    knob_position: u8,
    knob_values: [f32; 11],
    output_limit: (f32, f32),
}

impl Gain {
    pub fn new(knob_position: u8) -> Result<Self, String> {
        if knob_position > 10 {
            return Err(String::from("Incorrect knob value. Must be in range 0..10"));
        }

        Ok(Gain {
            knob_position,
            knob_values: [1.0, 1.6, 2.5, 4.0, 6.0, 10.0, 16.0, 25.0, 40.0, 63.0, 100.0],
            output_limit: (-0.95, 0.95),
        })
    }

    pub fn set_knob(&mut self, knob_position: u8) {
        self.knob_position = knob_position.min(10);
    }

    fn gain_value(&self) -> f32 {
        self.knob_values[self.knob_position as usize]
    }
}

impl AudioNode for Gain {
    fn process(&mut self, input: f32) -> f32 {
        (input * self.gain_value()).clamp(self.output_limit.0, self.output_limit.1)
    }
}
