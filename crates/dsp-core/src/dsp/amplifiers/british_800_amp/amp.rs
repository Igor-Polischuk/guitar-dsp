use crate::dsp::{
    MasterVolume, SampleProcessingNode,
    amplifiers::{
        amp_node::AmpNode,
        amp_param::KnobDescriptor,
        british_800_amp::{
            knobs::{BRITISH_800_INPUTS, BRITISH_800_KNOBS},
            params::{British800Input, British800Params},
            stages::{GainStage, InputStage, PowerAmpStage},
            tone_stack::MarshallToneStack,
        },
    },
};

/// Insparation is Marshall Lead JCM800
// input → couple of gain stages →
// cold clipper → cathode follower →
// tone stack → master volume →
// power amp/presence → cabinet IR
pub struct British800Amp {
    params: British800Params,

    input_stage: InputStage,
    gain_stage: GainStage,
    tone_stack: MarshallToneStack,
    power_amp_stage: PowerAmpStage,
    volume: MasterVolume,
}

impl SampleProcessingNode for British800Amp {
    fn process(&mut self, input: f32) -> f32 {
        let mut x = input;
        x = self.input_stage(x);
        x = self.gain_stage(x);
        x = self.tone_stack.process_sample(x);
        x = self.volume.process(x);
        x = self.power_amp_aprx(x);

        x
    }
}

impl AmpNode for British800Amp {
    fn knobs() -> &'static [KnobDescriptor] {
        BRITISH_800_KNOBS
    }

    fn inputs() -> &'static [crate::dsp::amplifiers::amp_param::InputDescriptor] {
        BRITISH_800_INPUTS
    }

    fn model_id(&self) -> &'static str {
        "british_800"
    }

    fn name(&self) -> &'static str {
        "British 800"
    }
}

impl British800Amp {
    pub fn new(sample_rate: f32, params: &British800Params) -> Self {
        let tone_stack = MarshallToneStack::new(
            sample_rate,
            params.bass.clone(),
            params.mid.clone(),
            params.treble.clone(),
        );

        British800Amp {
            params: params.clone(),
            input_stage: InputStage::new(sample_rate),
            gain_stage: GainStage::new(sample_rate),
            volume: MasterVolume::new(sample_rate, params.master_volume.clone()),
            tone_stack,
            power_amp_stage: PowerAmpStage::new(sample_rate, params.presence.clone()),
        }
    }

    fn input_stage(&mut self, input: f32) -> f32 {
        let mut x = input;
        if self.params.get_active_input() == British800Input::Low {
            x *= 0.5; // - 6dB, BTW +20 dB ≈  x10
            x = self.input_stage.hpf_low_input.process(x);
        } else {
            x = self.input_stage.hpf_high_input.process(x);
        }

        // TODO: bright shaping for high frs based on gain?

        x
    }

    fn gain_stage(&mut self, input: f32) -> f32 {
        let mut x = input;

        // Stage 1 lump gain and clipping
        x *= 45.0; // approximate value, based on schema it within 40-50
        x = x.tanh(); // soft clipping

        x = self.gain_stage.hpf_1.process(x);
        x = self.gain_stage.lpf_1.process(x);

        x *= self.params.pre_amp_volume.get(); // volume params from 0 to 1, determine how many of signal will we pass
        x *= 10.0;

        x = self.gain_stage.cold_clipper.process(x);

        x = self.gain_stage.hpf_2.process(x);
        x = self.gain_stage.lpf_2.process(x);

        // Stage 3
        x *= 7.0;
        x = x.tanh();

        // optional level compensation
        x *= 0.35;

        x = self.gain_stage.hpf_3.process(x);
        x = self.gain_stage.lpf_3.process(x);

        x
    }

    fn power_amp_aprx(&mut self, input: f32) -> f32 {
        let presence = self.params.presence.get().clamp(0.0, 1.0);
        let fb_raw = self.power_amp_stage.last_power_amp_output;
        let fb_high = self.power_amp_stage.presence_hpf.process(fb_raw);

        let fb_shaped = fb_raw - presence * fb_high;

        let feedback_amount = 0.35;
        let mut x = input - feedback_amount * fb_shaped;
        // x = self.power_amp_stage.hpf.process(x);

        x *= 5.0;
        x = x.tanh();

        // x = self.power_amp_stage.lpf.process(x);

        self.power_amp_stage.last_power_amp_output = x;

        x * 0.8
    }
}

// Input trim
// ↓
// Input HPF / bright shaping
// ↓
// Gain Stage 1
// ↓
// Coupling HPF
// ↓
// Preamp Volume / Gain knob
// ↓
// Gain Stage 2 / cold clipper
// ↓
// Coupling filter
// ↓
// Gain Stage 3
// ↓
// Cathode follower approximation
// ↓
// Marshall tone stack: Bass / Middle / Treble
// ↓
// Master Volume
// ↓
// Power amp approximation
// ↓
// Presence / resonance shaping
// ↓
// Cabinet IR
// ↓
// Output level

//малий coupling capacitor → менше низу
// великий coupling capacitor → більше низу
// cathode bypass capacitor → більше gain на певних частотах
// bright cap на volume pot → більше верху на малому gain
