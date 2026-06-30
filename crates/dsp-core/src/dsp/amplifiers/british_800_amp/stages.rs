use std::sync::Arc;

use crate::{
    dsp::{
        HighPassFilter, LowPassFilter, filters::PeakingFilter,
        helpers::dynamic_cold_clipper::DynamicColdClipper,
    },
    utils::AtomicF32,
};

pub struct InputStage {
    pub hpf_low_input: HighPassFilter,
    pub hpf_high_input: HighPassFilter,
}

impl InputStage {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            hpf_low_input: HighPassFilter::new(Arc::new(AtomicF32::new(100.0)), sample_rate),
            hpf_high_input: HighPassFilter::new(Arc::new(AtomicF32::new(45.0)), sample_rate),
        }
    }
}

pub struct GainStage {
    pub v1b_hpf: HighPassFilter,
    pub v1b_lpf: LowPassFilter,
    pub interstage_hpf: HighPassFilter,
    pub interstage_lpf: LowPassFilter,
    pub hpf_3: HighPassFilter,
    pub lpf_3: LowPassFilter,
    pub interstage_bright_hpf: HighPassFilter,
    pub cathode_follower_lpw: LowPassFilter,
    pub preamp_bright_hpf: HighPassFilter,
    pub cold_clipper: DynamicColdClipper,
    pub pre_cold_hpf: HighPassFilter,
}

impl GainStage {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            v1b_hpf: HighPassFilter::new(Arc::new(AtomicF32::new(15.0)), sample_rate),
            v1b_lpf: LowPassFilter::new(Arc::new(AtomicF32::new(16000.0)), sample_rate),
            interstage_hpf: HighPassFilter::new(Arc::new(AtomicF32::new(20.0)), sample_rate),
            interstage_lpf: LowPassFilter::new(Arc::new(AtomicF32::new(18000.0)), sample_rate),
            hpf_3: HighPassFilter::new(Arc::new(AtomicF32::new(10.0)), sample_rate),
            lpf_3: LowPassFilter::new(Arc::new(AtomicF32::new(16000.0)), sample_rate),
            interstage_bright_hpf: HighPassFilter::new(
                Arc::new(AtomicF32::new(720.0)),
                sample_rate,
            ),
            cathode_follower_lpw: LowPassFilter::new(
                Arc::new(AtomicF32::new(12000.0)),
                sample_rate,
            ),
            preamp_bright_hpf: HighPassFilter::new(Arc::new(AtomicF32::new(1500.0)), sample_rate),
            cold_clipper: DynamicColdClipper::new(sample_rate, -0.65),
            pre_cold_hpf: HighPassFilter::new(Arc::new(AtomicF32::new(80.0)), sample_rate),
        }
    }
}

pub struct PowerAmpStage {
    pub hpf: HighPassFilter,
    pub lpf: LowPassFilter,
    pub last_power_amp_output: f32,
    pub presence_hpf: HighPassFilter,
    pub upper_mid_notch: PeakingFilter,
    pub driven_upper_mid_notch: PeakingFilter,
    pub driven_upper_mid_focus_notch: PeakingFilter,
    pub upper_harmonic_hpf: HighPassFilter,
}

impl PowerAmpStage {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            hpf: HighPassFilter::new(Arc::new(AtomicF32::new(35.0)), sample_rate),
            presence_hpf: HighPassFilter::new(Arc::new(AtomicF32::new(1000.0)), sample_rate),
            lpf: LowPassFilter::new(Arc::new(AtomicF32::new(15000.0)), sample_rate),
            last_power_amp_output: 0.0,
            upper_mid_notch: PeakingFilter::new(sample_rate, 1800.0, -6.0, 1.15),
            driven_upper_mid_notch: PeakingFilter::new(sample_rate, 1800.0, -18.0, 2.0),
            driven_upper_mid_focus_notch: PeakingFilter::new(sample_rate, 1600.0, -36.0, 8.0),
            upper_harmonic_hpf: HighPassFilter::new(Arc::new(AtomicF32::new(900.0)), sample_rate),
        }
    }
}
