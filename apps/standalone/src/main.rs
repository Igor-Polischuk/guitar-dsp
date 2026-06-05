use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use dsp_engine::prelude::*;
use ringbuf::HeapRb;
use ringbuf::traits::{Consumer, Producer, Split};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = cpal::default_host(); // OS audio system interface
    let (input_device, output_device) = get_audio_devices(&host)?;

    let input_config: cpal::StreamConfig = input_device.default_input_config()?.into();
    println!("Config: {:?}", input_config);

    let output_config: cpal::StreamConfig = output_device.default_output_config()?.into();
    println!("output_config: {:?}", output_config);

    let rb = HeapRb::<f32>::new(48_000); // 1 second buffer at 48kHz, consider decrease if latency is an issue
    let (producer, consumer) = rb.split();

    let input_stream = process_input(input_device, input_config, producer)?;
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
) -> Result<Stream, Box<dyn std::error::Error>>
where
    P: Producer<Item = f32> + Send + 'static,
{
    let input_channels = input_config.channels as usize;
    let mut processing_chain = get_processing_chain(input_config.sample_rate as f32);
    let stream = input_device.build_input_stream(
        &input_config,
        move |data: &[f32], _| {
            for frame in data.chunks(input_channels) {
                let raw = if input_channels >= 2 {
                    (frame[0] + frame[1]) * 0.5
                } else {
                    frame[0]
                };

                let processed = processing_chain.process(raw);
                _ = producer.try_push(processed);
            }
        },
        move |err| eprintln!("Input error: {err}"),
        None,
    )?;

    Ok(stream)
}

fn get_processing_chain(sample_rate: f32) -> SignalChain {
    println!("Initing sound processing chain");
    let mut processing_chain = SignalChain::new();
    let gain = Gain::new(6).unwrap();
    let distortion = Distortion::new(DistortionPreset::Crunch);
    let high_pass_filter = HighPassFilter::new(70.0, sample_rate); // for clean, 120 for high gain
    let low_pass_filter = LowPassFilter::new(8000.0, sample_rate);
    let volume = MasterVolume::new(sample_rate);

    let cabinet_manager = CabinetManager::<1024>::new(sample_rate);
    let cabinet = cabinet_manager.get_cabinet(Cabinet::CenzoCelestion);

    let mut eq = Equalizer::new(sample_rate);
    eq.set_bass_knob(7);
    eq.set_mid_knob(2);
    eq.set_treble_knob(8);

    processing_chain.append_node(high_pass_filter);
    processing_chain.append_node(gain);
    // processing_chain.append_node(distortion);
    processing_chain.append_node(eq);
    processing_chain.append_node(cabinet);
    processing_chain.append_node(low_pass_filter);
    processing_chain.append_node(volume);

    processing_chain
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
