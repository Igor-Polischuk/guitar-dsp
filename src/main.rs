use std::sync::mpsc;
use std::thread;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use ringbuf::HeapRb;
use ringbuf::traits::{Consumer, Producer, Split};

mod processing_pipeline;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host(); // OS audio system interface
    let (input_device, output_device) = get_audio_devices(&host)?;

    let input_config: cpal::StreamConfig = input_device.default_input_config()?.into();
    println!("Config: {:?}", input_config);

    let output_config: cpal::StreamConfig = output_device.default_output_config()?.into();
    println!("output_config: {:?}", output_config);

    let rb = HeapRb::<f32>::new(48_000); // 1 second buffer at 48kHz, consider decrease if latency is an issue
    let (producer, consumer) = rb.split();

    let sample_rate = input_config.sample_rate as f32;
    let (pitch_tx, pitch_rx) = mpsc::sync_channel::<Vec<f32>>(1);

    thread::spawn(move || {
        while let Ok(samples) = pitch_rx.recv() {
            if let Some(freq) = detect_pitch(&samples, sample_rate) {
                let note = hz_to_note(freq).unwrap_or(String::from("Unknown"));
                // println!("frequency: {freq:.2} Hz. It is {note} note");
            }
        }
    });

    let input_stream = process_input(input_device, input_config, producer, pitch_tx)?;
    let output_stream = process_output(output_device, output_config, consumer)?;

    input_stream.play()?;
    output_stream.play()?;

    println!("Listening... Press Enter to stop.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}

fn get_audio_devices(
    host: &cpal::Host,
) -> Result<(cpal::Device, cpal::Device), Box<dyn std::error::Error>> {
    // todo refactor to let user select devices from list
    let input_device = host
        .input_devices()?
        .find(|d| {
            d.name()
                .map(|name| name.contains("Volt 1"))
                .unwrap_or(false)
        })
        .expect("Device not found");

    println!("Input device: {}", input_device.description()?);

    let output_device = host
        .default_output_device()
        .expect("Output device not found");

    println!("Output: {}", output_device.description()?);

    Ok((input_device, output_device))
}

fn process_input<P>(
    input_device: Device,
    input_config: StreamConfig,
    mut producer: P,
    pitch_tx: mpsc::SyncSender<Vec<f32>>,
) -> Result<Stream, Box<dyn std::error::Error>>
where
    P: Producer<Item = f32> + Send + 'static,
{
    let input_channels = input_config.channels as usize;
    let mut pitch_buffer = Vec::with_capacity(4096); // 5120 samples for 1 second at 48kHz
    let stream = input_device.build_input_stream(
        &input_config,
        move |data: &[f32], _| {
            for frame in data.chunks(input_channels) {
                let raw = frame[0];

                pitch_buffer.push(raw);
                let mut processor = processing_pipeline::ProcessingPipeline::new(raw);

                if pitch_buffer.len() >= 4096 {
                    let samples = std::mem::take(&mut pitch_buffer);
                    let _ = pitch_tx.try_send(samples);
                    pitch_buffer = Vec::with_capacity(4096);
                }

                // let processed = (raw * 50.0).clamp(-1.0, 1.0); //VOLUME (GAIN)
                let processor = processor.apply_gain(10.).apply_distortion();
                _ = producer.try_push(processor.get_output());
            }
        },
        move |err| eprintln!("Input error: {err}"),
        None,
    )?;

    Ok(stream)
}

fn process_output<C>(
    output_device: Device,
    output_config: StreamConfig,
    mut consumer: C,
) -> Result<Stream, Box<dyn std::error::Error>>
where
    C: Consumer<Item = f32> + Send + 'static,
{
    let output_channels = output_config.channels as usize;

    let output_stream = output_device.build_output_stream(
        &output_config,
        move |data: &mut [f32], _| {
            for frame in data.chunks_mut(output_channels) {
                let sample = consumer.try_pop().unwrap_or(0.0);

                for out in frame {
                    *out = sample;
                }
            }
        },
        move |err| eprintln!("Output error: {err}"),
        None,
    )?;

    Ok(output_stream)
}

fn detect_pitch(samples: &[f32], sample_rate: f32) -> Option<f32> {
    let half_len = samples.len() / 2;
    if half_len < 2 {
        return None;
    }

    let mut diff: Vec<f32> = Vec::with_capacity(half_len);

    // STEP 1: Squared Difference
    for lag in 0..half_len {
        let mut sum = 0.0;
        for i in 0..half_len {
            let curr_diff = (samples[i] - samples[i + lag]).powi(2);
            sum += curr_diff;
        }
        diff.push(sum);
    }

    // STEP 2: Cumulative Mean Normalized Difference
    let mut running_sum = 0.0;
    diff[0] = 1.0;

    for tau in 1..half_len {
        running_sum += diff[tau];
        if running_sum != 0.0 {
            diff[tau] /= running_sum / (tau as f32);
        } else {
            diff[tau] = 1.0;
        }
    }

    // STEP 3: Пошук першого мінімуму нижче порогу
    let threshold = 0.15; // Стандартний поріг для YIN (можна крутити 0.1 - 0.15)
    let mut candidate_index = None;

    // Шукаємо перший локальний мінімум, який менший за threshold
    for i in 1..(half_len - 1) {
        if diff[i] < threshold {
            // Перевіряємо, чи це локальний мінімум (менший за сусідів)
            if diff[i] < diff[i - 1] && diff[i] < diff[i + 1] {
                candidate_index = Some(i);
                break; // Знайшли перший найкращий період — виходимо!
            }
        }
    }

    // Якщо явного мінімуму нижче порогу немає, беремо просто абсолютний мінімум
    let candidate_index = candidate_index.unwrap_or_else(|| {
        let mut min_idx = 1;
        for i in 2..(half_len - 1) {
            if diff[i] < diff[min_idx] {
                min_idx = i;
            }
        }
        min_idx
    });

    // Захист від занадто малих лагів (щоб не ділити на 0 і не отримати артефакти)
    if candidate_index < 2 || candidate_index >= half_len - 1 {
        return None;
    }

    // STEP 4: Правильна параболічна інтерполяція (Standard parabolic fit)
    let alpha = diff[candidate_index - 1];
    let beta = diff[candidate_index];
    let gamma = diff[candidate_index + 1];

    let denominator = 2.0 * (alpha - 2.0 * beta + gamma);
    let delta = if denominator != 0.0 {
        (alpha - gamma) / denominator
    } else {
        0.0
    };

    // Додаємо дельту до ІНДЕКСУ (часу затримки), а не до значення функції
    let period = candidate_index as f32 + delta;

    if period > 0.0 {
        let freq = sample_rate / period;
        Some(freq)
    } else {
        None
    }
}

fn hz_to_note(freq: f32) -> Option<String> {
    if freq <= 0.0 {
        return None;
    }

    // 1. Рахуємо MIDI-номер (дозволяємо дробове значення для точності)
    let midi_num = 69.0 + 12.0 * (freq / 440.0).log2();

    // Округляємо до найближчого цілого півтону
    let midi_round = midi_num.round() as i32;

    // Перевіряємо, чи є нота в межах адекватного аудіодіапазону
    if midi_round < 0 || midi_round > 127 {
        return None;
    }

    // Назви нот у межах однієї октави (всього 12 півтонів)
    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    // Визначаємо індекс ноти в октаві та номер самої октави
    let note_index = (midi_round % 12) as usize;
    let octave = (midi_round / 12) - 1; // MIDI нота 0 — це октава -1

    Some(format!("{}{}", note_names[note_index], octave))
}

// fn detect_pitch(samples: &[f32], sample_rate: f32) -> Option<f32> {
//     // STEP 1: Squared Difference
//     let half_len = samples.len() / 2;
//     let mut diff: Vec<f32> = Vec::with_capacity(half_len);

//     for lag in 0..half_len {
//         let mut sum = 0.0;
//         for i in 0..half_len {
//             let curr_diff = (samples[i] - samples[i + lag]).powi(2); // samples[i+lag] не вийде за 2048
//             sum += curr_diff;
//         }
//         diff.push(sum);
//     }

//     // STEP 2: Normalisation
//     let mut running_sum = diff[0];
//     diff[0] = 1.0;

//     for i in 1..diff.len() {
//         running_sum += diff[i];

//         let count = (i + 1) as f32;
//         let mean = running_sum / count;

//         if mean != 0.0 {
//             diff[i] /= mean;
//         } else {
//             diff[i] = 1.0; // Захист на випадок, якщо весь сигнал — це абсолютна тиша (всі нулі)
//         }
//     }

//     let threshold = 0.1;

//     let mut candidate_index = 0;

//     for i in 0..diff.len() {
//         if diff[i] < threshold {
//             candidate_index = i;
//         }
//     }

//     if candidate_index == 0 {
//         return None;
//     }

//     let delta = (diff[candidate_index + 1] - diff[candidate_index - 1])
//         / (2.0
//             * (2.0
//                 * diff[candidate_index]
//                 * diff[candidate_index - 1]
//                 * diff[candidate_index + 1]));

//     let period = diff[candidate_index] + delta;

//     let freq = sample_rate / period;

//     Some(freq)
// }
