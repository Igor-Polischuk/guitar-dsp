use crate::dsp::AudioNode;

#[derive(Clone, Copy, Debug)]
pub enum DistortionPreset {
    SmoothTube,
    LightValve,
    Crunch,
    Transistor,
}

pub struct Distortion {
    positive_drive: f32,
    negative_drive: f32,
    saturation_amount: f32,
}

impl Distortion {
    pub fn new(preset: DistortionPreset) -> Self {
        let mut distortion = Distortion {
            positive_drive: 1.0,
            negative_drive: 1.0,
            saturation_amount: 1.0,
        };
        distortion.apply_preset(preset);
        distortion
    }

    pub fn apply_preset(&mut self, preset: DistortionPreset) {
        let (positive_drive, negative_drive, saturation_amount) = match preset {
            DistortionPreset::SmoothTube => (2.1, 0.45, 1.3),
            DistortionPreset::LightValve => (1.5, 0.9, 1.2),
            DistortionPreset::Crunch => (3.1, 0.75, 1.5),
            DistortionPreset::Transistor => (2.0, 1.0, 2.0),
        };

        self.positive_drive = positive_drive;
        self.negative_drive = negative_drive;
        self.saturation_amount = saturation_amount;
    }
}

impl AudioNode for Distortion {
    fn process(&mut self, input: f32) -> f32 {
        // if input < 0.0 {
        //     self.negative_drive * (self.saturation_amount * input).tanh()
        // } else {
        //     (self.positive_drive * input).tanh()
        // }

        if input < 0.0 {
            (input * self.negative_drive * self.saturation_amount).tanh()
        } else {
            (input * self.positive_drive * self.saturation_amount).tanh()
        }
    }
}
