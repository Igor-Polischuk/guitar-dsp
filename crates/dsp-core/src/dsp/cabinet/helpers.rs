use std::io::Cursor;

pub fn fit_to_length<const N: usize>(ir: Vec<f32>) -> [f32; N] {
    let mut arr = [0.0; N];

    for (src, dest) in ir.iter().zip(arr.iter_mut()) {
        *dest = *src;
    }

    arr
}

pub fn fade_out_tail<const N: usize>(ir: &mut [f32; N], fade_len: usize) {
    let len = ir.len();

    if fade_len == 0 || fade_len >= len {
        return;
    }

    let start = len - fade_len;

    for i in 0..fade_len {
        let t = i as f32 / fade_len as f32;
        let gain = 1.0 - t;
        ir[start + i] *= gain;
    }
}

pub fn remove_dc_offset(mut ir: Vec<f32>) -> Vec<f32> {
    if ir.is_empty() {
        return ir;
    }

    let mean = ir.iter().sum::<f32>() / ir.len() as f32;

    for sample in &mut ir {
        *sample -= mean;
    }

    ir
}

pub fn trim_leading_silence(ir: Vec<f32>) -> Vec<f32> {
    let threshold = 0.001;

    let start = ir.iter().position(|s| s.abs() > threshold).unwrap_or(0);

    ir[start..].to_vec()
}

pub fn apply_gain(mut ir: Vec<f32>, gain: f32) -> Vec<f32> {
    for sample in ir.iter_mut() {
        *sample *= gain;
    }
    ir
}

pub fn normalize_peak(mut ir: Vec<f32>) -> Vec<f32> {
    let max_peak = ir
        .iter()
        .map(|s| s.abs())
        .fold(0.0, |max, val| if val > max { val } else { max });

    if max_peak <= 0.0 {
        return ir;
    }

    for index in 0..ir.len() {
        ir[index] /= max_peak;
    }

    ir
}

pub fn load_embedded_response(
    response_bytes: &[u8],
    target_sample_rate: f32,
) -> Result<Vec<f32>, String> {
    let cursor = Cursor::new(response_bytes);

    let mut reader = hound::WavReader::new(cursor)
        .map_err(|err| format!("Failed to read cabinet response WAV: {err}"))?;

    let spec = reader.spec();

    if spec.sample_rate as f32 != target_sample_rate {
        return Err(format!(
            "Cabinet response sample rate mismatch: file={}Hz, engine={}Hz",
            spec.sample_rate, target_sample_rate
        ));
    }

    let channels = spec.channels as usize;

    let mut samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .samples::<f32>()
            .map(|sample| sample.unwrap_or(0.0))
            .collect(),

        hound::SampleFormat::Int => {
            let max_value = (1 << (spec.bits_per_sample - 1)) as f32;

            if spec.bits_per_sample <= 16 {
                reader
                    .samples::<i16>()
                    .map(|sample| sample.unwrap_or(0) as f32 / max_value)
                    .collect()
            } else {
                reader
                    .samples::<i32>()
                    .map(|sample| sample.unwrap_or(0) as f32 / max_value)
                    .collect()
            }
        }
    };

    if channels > 1 {
        samples = mixdown_to_mono(samples, channels);
    }

    Ok(samples)
}

fn mixdown_to_mono(samples: Vec<f32>, channels: usize) -> Vec<f32> {
    let mut mono = Vec::with_capacity(samples.len() / channels);

    for frame in samples.chunks_exact(channels) {
        let sum: f32 = frame.iter().sum();
        mono.push(sum / channels as f32);
    }

    mono
}
