use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use dsp_core::dsp::{British800Amp, British800Params, SampleProcessingNode};

const INPUT_FILES: &[&str] = &[
    "apps/wav-renderer/test-wav/Sine 440 by xserra.wav",
    "apps/wav-renderer/test-wav/chords.wav",
    "apps/wav-renderer/test-wav/power.wav",
    "apps/wav-renderer/test-wav/palm.wav",
    "apps/wav-renderer/test-wav/notes.wav",
    "apps/wav-renderer/test-wav/Dry Guitar Track.wav",
    "apps/wav-renderer/test-wav/High Quality Pink Noise.wav",
];

const OUTPUT_DIR: &str = "apps/wav-renderer/processed-wav";

fn set_up_processor(sample_rate: f32) -> British800Amp {
    let amp_params = British800Params::default();

    amp_params.set("pre_amp_volume", 1.0);
    amp_params.set("master_volume", 10.0);
    amp_params.set("presence", 0.0);
    amp_params.set("treble", 5.0);
    amp_params.set("mid", 5.0);
    amp_params.set("bass", 5.0);

    British800Amp::new(sample_rate, &amp_params)
}

fn main() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(OUTPUT_DIR)?;

    for input_path in INPUT_FILES {
        println!("Process {input_path}");
        process_wav(Path::new(input_path))?;
        println!("-----------------------")
    }

    Ok(())
}

fn process_wav(input_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut reader = hound::WavReader::open(input_path)?;
    let spec = reader.spec();

    println!("{:?}", spec);

    let sample_rate = spec.sample_rate as f32;

    let mut amp = set_up_processor(sample_rate);

    let file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let output_path = PathBuf::from(OUTPUT_DIR).join(format!("{file_stem}_british800.wav"));

    let output_spec = hound::WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(output_path, output_spec)?;

    let samples = read_samples_as_f32(&mut reader, spec)?;
    println!("{}", rms(samples.clone()));
    println!("{}", peak(samples.clone()));

    for sample in samples {
        let y = amp.process(sample);

        writer.write_sample(y)?;
    }

    writer.finalize()?;

    Ok(())
}

fn read_samples_as_f32<R: std::io::Read>(
    reader: &mut hound::WavReader<R>,
    spec: hound::WavSpec,
) -> Result<Vec<f32>, Box<dyn Error>> {
    let samples = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().collect::<Result<Vec<_>, _>>()?,

        hound::SampleFormat::Int => match spec.bits_per_sample {
            16 => reader
                .samples::<i16>()
                .map(|s| s.map(|v| v as f32 / i16::MAX as f32))
                .collect::<Result<Vec<_>, _>>()?,

            24 | 32 => {
                let max = ((1_i64 << (spec.bits_per_sample - 1)) - 1) as f32;

                reader
                    .samples::<i32>()
                    .map(|s| s.map(|v| v as f32 / max))
                    .collect::<Result<Vec<_>, _>>()?
            }

            _ => {
                return Err(format!("Unsupported int bit depth: {}", spec.bits_per_sample).into());
            }
        },
    };

    Ok(samples)
}

fn rms(samples: Vec<f32>) -> f32 {
    let sum = samples.iter().map(|x| x * x).sum::<f32>();
    (sum / samples.len() as f32).sqrt()
}

fn peak(samples: Vec<f32>) -> f32 {
    samples.iter().fold(0.0, |max, x| max.max(x.abs()))
}
