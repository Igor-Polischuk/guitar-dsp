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
    helpers::utils::{db_to_gain, tube_stage, tube_stage_unity},
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

    output_asym_hpf_alpha: f32,
    output_asym_hpf_x1: f32,
    output_asym_hpf_y1: f32,
    input_level_env: f32,
}

impl SampleProcessingNode for British800Amp {
    fn process(&mut self, input: f32) -> f32 {
        self.update_input_level(input);

        let mut x = input;
        x = self.input_stage(x);
        x = self.gain_stage(x);
        x = self.tone_stack.process_sample(x);
        x = self.volume.process(x);
        x = self.phase_inverter_aprx(x);
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

        let output_asym_hpf_cutoff_hz = 520.0;
        let rc = 1.0 / (2.0 * std::f32::consts::PI * output_asym_hpf_cutoff_hz);
        let dt = 1.0 / sample_rate;
        let output_asym_hpf_alpha = rc / (rc + dt);

        British800Amp {
            params: params.clone(),
            input_stage: InputStage::new(sample_rate),
            gain_stage: GainStage::new(sample_rate),
            volume: MasterVolume::new(sample_rate, params.master_volume.clone()),
            tone_stack,
            power_amp_stage: PowerAmpStage::new(sample_rate),
            output_asym_hpf_alpha,
            output_asym_hpf_x1: 0.0,
            output_asym_hpf_y1: 0.0,
            input_level_env: 0.0,
        }
    }

    fn update_input_level(&mut self, input: f32) {
        let target = input.abs();
        let coeff = if target > self.input_level_env {
            0.002
        } else {
            0.0002
        };

        self.input_level_env += coeff * (target - self.input_level_env);
    }

    fn input_stage(&mut self, input: f32) -> f32 {
        let mut x = input;
        if self.params.get_active_input() == British800Input::Low {
            x = self.input_stage.hpf_low_input.process(x);
        } else {
            x = self.input_stage.hpf_high_input.process(x);
        }

        x
    }

    fn gain_stage(&mut self, input: f32) -> f32 {
        let mut x = input;
        if self.params.get_active_input() == British800Input::High {
            x = self.v1b_stage(x);
        }

        // Preamp Volume + bright cap
        x = self.preamp_volume_stage(x);

        x = self.gain_stage.pre_cold_hpf.process(x);
        x = self.gain_stage.cold_clipper.process(x * 8.0) * 1.1;
        x = self.interstage_after_cold_clipper(x);

        x = self.v2a_stage(x);

        x = self.cathode_follower(x);

        x
    }

    fn v1b_stage(&mut self, input: f32) -> f32 {
        let mut x = input;

        // first gain triode
        x = tube_stage(x, 18.0, -0.03, 1.2);

        x = self.gain_stage.v1b_hpf.process(x);
        x = self.gain_stage.v1b_lpf.process(x);

        x
    }

    fn preamp_volume_stage(&mut self, input: f32) -> f32 {
        let knob = self.params.pre_amp_volume.get();

        if knob <= 0.001 {
            return 0.0;
        }

        let pot = knob.powf(1.6);

        let main = input * pot;

        let bright_weight = knob.powf(0.7) * (1.0 - knob).powf(1.6);

        let bright = self.gain_stage.preamp_bright_hpf.process(input) * 0.18 * bright_weight;

        main + bright
    }

    fn v2a_stage(&mut self, input: f32) -> f32 {
        let mut x = input;

        // third gain triode after cold clipper
        x = tube_stage(x, 12.0, -0.06, 1.45);

        x = self.gain_stage.hpf_3.process(x);
        x = self.gain_stage.lpf_3.process(x);

        x
    }

    fn interstage_after_cold_clipper(&mut self, input: f32) -> f32 {
        let ac = self.gain_stage.interstage_hpf.process(input);
        let low_path = ac * db_to_gain(-6.0);

        let bright_path = self.gain_stage.interstage_bright_hpf.process(ac) * 0.5;

        let mut y = low_path + bright_path;

        y = self.gain_stage.interstage_lpf.process(y);

        y
    }

    fn cathode_follower(&mut self, input: f32) -> f32 {
        let drive = 1.2;
        let bias = 0.1;

        let mut x = (input * drive + bias).tanh() - bias.tanh();

        x = self.gain_stage.cathode_follower_lpw.process(x);

        x * 0.83
    }

    fn power_amp_aprx(&mut self, input: f32) -> f32 {
        let presence = self.params.presence.get().clamp(0.0, 1.0);

        let mut x = input;

        let fb_raw = self.power_amp_stage.last_power_amp_output;
        let fb_high = self.power_amp_stage.presence_hpf.process(fb_raw);
        let fb_shaped = fb_raw - presence * fb_high;

        let feedback_amount = 0.2;
        x = x - feedback_amount * fb_shaped;

        x = self.power_amp_stage.hpf.process(x);

        // light glue only, no big level boost
        x = tube_stage_unity(x, 2.0, 0.03, 0.10);

        x = self.power_amp_stage.lpf.process(x);
        x = self.power_amp_stage.upper_mid_notch.process(x);
        x = self.driven_upper_mid_notch(x);
        x = self.driven_upper_mid_focus_notch(x);
        x = self.upper_harmonic_saturation(x);
        x = self.output_saturation(x);

        self.power_amp_stage.last_power_amp_output = x;

        x
    }

    fn upper_harmonic_saturation(&mut self, input: f32) -> f32 {
        let hp = self.power_amp_stage.upper_harmonic_hpf.process(input);
        let bias = 0.18;
        let harmonic = (hp * 7.0 + bias).tanh() - bias.tanh();

        input + harmonic * 0.06
    }

    fn driven_upper_mid_notch(&mut self, input: f32) -> f32 {
        let notched = self.power_amp_stage.driven_upper_mid_notch.process(input);
        let mix = self.driven_upper_mid_mix();

        input + (notched - input) * mix
    }

    fn driven_upper_mid_focus_notch(&mut self, input: f32) -> f32 {
        let notched = self
            .power_amp_stage
            .driven_upper_mid_focus_notch
            .process(input);
        let mix = self.driven_upper_mid_dynamic_mix();

        input + (notched - input) * mix
    }

    fn driven_upper_mid_mix(&self) -> f32 {
        let dynamic = ((self.input_level_env - 0.08) / 0.18).clamp(0.0, 1.0);

        0.25 + 0.75 * dynamic
    }

    fn driven_upper_mid_dynamic_mix(&self) -> f32 {
        ((self.input_level_env - 0.08) / 0.18).clamp(0.0, 1.0)
    }

    fn output_saturation(&mut self, input: f32) -> f32 {
        let clipped = (input * 7.0).tanh() * 0.48;

        let hp = self.output_asym_hpf_alpha
            * (self.output_asym_hpf_y1 + clipped - self.output_asym_hpf_x1);
        self.output_asym_hpf_x1 = clipped;
        self.output_asym_hpf_y1 = hp;

        let asymmetric_lift = 0.65 * hp * hp;

        (clipped + asymmetric_lift).clamp(-0.72, 0.72)
    }

    fn phase_inverter_aprx(&mut self, input: f32) -> f32 {
        tube_stage_unity(input, 3.0, 0.08, 0.35)
    }
}
