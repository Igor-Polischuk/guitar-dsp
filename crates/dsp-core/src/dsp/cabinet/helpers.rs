use std::io::Cursor;

pub fn transform_ir<const N: usize>(ir: Vec<f32>) -> [f32; N] {
    let mut arr = [0.0; N];

    for (src, dest) in ir.iter().zip(arr.iter_mut()) {
        *dest = *src;
    }

    arr
}

pub fn peak_normalize(mut ir: Vec<f32>) -> Vec<f32> {
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

// split function
pub fn load_embedded_ir(ir_bytes: &[u8], target_sample_rate: f32) -> Result<Vec<f32>, String> {
    let cursor = Cursor::new(ir_bytes);

    // Remove unwrap
    let mut reader = hound::WavReader::new(cursor).unwrap();

    let spec = reader.spec();

    if spec.sample_rate as f32 != target_sample_rate {
        return Err(format!(
            "File freq ({} Hz) not match with active one: ({} Hz)!",
            spec.sample_rate, target_sample_rate
        ));
    }

    let channels = spec.channels as usize;

    let mut raw_samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap_or(0.0)).collect(),
        hound::SampleFormat::Int => {
            let max_val = (1 << (spec.bits_per_sample - 1)) as f32;

            if spec.bits_per_sample <= 16 {
                reader
                    .samples::<i16>()
                    .map(|s| (s.unwrap_or(0) as f32) / max_val)
                    .collect()
            } else {
                reader
                    .samples::<i32>()
                    .map(|s| (s.unwrap_or(0) as f32) / max_val)
                    .collect()
            }
        }
    };

    if channels > 1 {
        let mut mono_samples = Vec::with_capacity(raw_samples.len() / channels);
        for frame in raw_samples.chunks_exact(channels) {
            let sum: f32 = frame.iter().sum();
            mono_samples.push(sum / (channels as f32));
        }
        raw_samples = mono_samples;
    }

    Ok(raw_samples)
}
