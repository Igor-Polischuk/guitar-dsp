use std::f32;

use super::HighPassFilter;
use crate::dsp::AudioNode;

const SAMPLE_RATE: f32 = 48_000.0;
const TEST_SAMPLES: usize = 48_000;
const WARMUP_SAMPLES: usize = 4_800;

fn sine_sample(frequency: f32, sample_index: usize) -> f32 {
    let phase = 2.0 * f32::consts::PI * frequency * sample_index as f32 / SAMPLE_RATE;
    phase.sin()
}

fn filtered_rms(filter: &mut HighPassFilter, frequency: f32) -> f32 {
    let mut sum = 0.0;
    let mut count = 0;

    for sample_index in 0..TEST_SAMPLES {
        let output = filter.process(sine_sample(frequency, sample_index));

        if sample_index >= WARMUP_SAMPLES {
            sum += output * output;
            count += 1;
        }
    }

    (sum / count as f32).sqrt()
}

#[test]
fn attenuates_signal_below_cutoff_more_than_signal_above_cutoff() {
    let cutoff = 180.0;

    let low_frequency_rms = filtered_rms(&mut HighPassFilter::new(cutoff, SAMPLE_RATE), 40.0);
    let high_frequency_rms = filtered_rms(&mut HighPassFilter::new(cutoff, SAMPLE_RATE), 1_000.0);

    assert!(
        low_frequency_rms < high_frequency_rms * 0.25,
        "expected 40 Hz RMS ({low_frequency_rms}) to be much lower than 1000 Hz RMS ({high_frequency_rms})"
    );
    assert!(
        high_frequency_rms > 0.60,
        "expected 1000 Hz RMS ({high_frequency_rms}) to pass through the HPF"
    );
}

#[test]
fn removes_dc_offset_after_filter_warmup() {
    let mut filter = HighPassFilter::new(80.0, SAMPLE_RATE);
    let mut sum = 0.0;
    let mut count = 0;

    for sample_index in 0..TEST_SAMPLES {
        let output = filter.process(1.0);

        if sample_index >= WARMUP_SAMPLES {
            sum += output * output;
            count += 1;
        }
    }

    let tail_rms = (sum / count as f32).sqrt();

    assert!(
        tail_rms < 0.001,
        "expected DC offset to be removed after warmup, got tail RMS {tail_rms}"
    );
}
