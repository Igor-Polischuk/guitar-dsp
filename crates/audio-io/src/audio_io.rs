use crate::device_manager::{AudioIoDevice, DeviceManager};
use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, Stream, StreamConfig};
use ringbuf::HeapRb;
use ringbuf::traits::Split;
use ringbuf::traits::{Consumer, Producer};

pub struct AudioIO {
    device_manager: DeviceManager,
    input_stream: Option<Stream>,
    output_stream: Option<Stream>,
    config: Option<ResolvedAudioConfig>,
}

struct ResolvedAudioConfig {
    input: ResolvedStreamConfig,
    output: ResolvedStreamConfig,
    sample_rate: SampleRate,
}

struct ResolvedStreamConfig {
    stream_config: StreamConfig,
    sample_format: SampleFormat,
}

pub struct AudioContext {
    pub sample_rate: SampleRate,
    pub input_channels: u16,
    pub output_channels: u16,
    pub ring_buffer_capacity: usize,
}

pub struct AudioIoSettings {
    pub input_device_name: Option<String>,
    pub output_device_name: Option<String>,
    pub preferred_sample_rates: Vec<u32>,
    pub ring_buffer_size: usize,
}

impl Default for AudioIoSettings {
    fn default() -> Self {
        AudioIoSettings {
            input_device_name: None,
            output_device_name: None,
            preferred_sample_rates: vec![48000, 44100],
            ring_buffer_size: 8192,
        }
    }
}

impl AudioIO {
    pub fn init() -> Self {
        AudioIO {
            device_manager: DeviceManager::new(),
            input_stream: None,
            output_stream: None,
            config: None,
        }
    }

    pub fn start<F, P>(
        &mut self,
        settings: AudioIoSettings,
        processor_factory: F,
    ) -> Result<(), String>
    where
        F: FnOnce(AudioContext) -> P,
        P: FnMut(&mut [f32]) + Send + 'static,
    {
        if self.is_running() {
            return Err("AudioIO is already running. Call stop() or restart() first".to_string());
        }

        if let Some(input_device) = settings.input_device_name {
            self.set_input_device(input_device.as_str())?;
        }

        if let Some(output_device) = settings.output_device_name {
            self.set_output_device(output_device.as_str())?;
        }

        if self.device_manager.active_input_device.is_none() {
            self.device_manager.set_default_input()?;
        }

        if self.device_manager.active_output_device.is_none() {
            self.device_manager.set_default_output()?;
        }

        self.resolve_configs(settings.preferred_sample_rates)?;

        let config = self
            .config
            .as_ref()
            .ok_or("Config not resolved during starting")?;

        self.build_streams(
            processor_factory(AudioContext {
                sample_rate: config.sample_rate,
                input_channels: config.input.stream_config.channels,
                output_channels: config.output.stream_config.channels,
                ring_buffer_capacity: settings.ring_buffer_size,
            }),
            settings.ring_buffer_size,
        )?;

        self.play()?;

        Ok(())
    }

    pub fn stop(&mut self) {
        self.input_stream = None;
        self.output_stream = None;
        self.config = None;
    }

    pub fn is_running(&self) -> bool {
        self.input_stream.is_some() && self.output_stream.is_some()
    }

    pub fn restart<F, P>(
        &mut self,
        settings: AudioIoSettings,
        processor_factory: F,
    ) -> Result<(), String>
    where
        F: FnOnce(AudioContext) -> P,
        P: FnMut(&mut [f32]) + Send + 'static,
    {
        self.stop();
        self.start(settings, processor_factory)
    }

    pub fn available_devices(&self) -> Result<Vec<AudioIoDevice>, String> {
        self.device_manager.get_available_devices()
    }

    pub fn set_input_device(&mut self, name: &str) -> Result<(), String> {
        self.device_manager.set_input_device(name)
    }

    pub fn set_output_device(&mut self, name: &str) -> Result<(), String> {
        self.device_manager.set_output_device(name)
    }

    fn build_streams<D>(
        &mut self,
        process_callback: D,
        ring_buffer_capacity: usize,
    ) -> Result<(), String>
    where
        D: FnMut(&mut [f32]) + Send + 'static,
    {
        let buffer = HeapRb::<f32>::new(ring_buffer_capacity);
        let (producer, consumer) = buffer.split();
        match self
            .config
            .as_ref()
            .ok_or("No config resolved")?
            .input
            .sample_format
        {
            SampleFormat::F32 => self.build_input_stream::<f32, _>(producer),
            SampleFormat::I16 => self.build_input_stream::<i16, _>(producer),
            SampleFormat::U16 => self.build_input_stream::<u16, _>(producer),
            format => return Err(format!("Unsupported input sample format: {format:?}")),
        }?;

        match self
            .config
            .as_ref()
            .ok_or("No config resolved")?
            .output
            .sample_format
        {
            SampleFormat::F32 => self.build_output_stream::<f32, _, _>(process_callback, consumer),
            SampleFormat::I16 => self.build_output_stream::<i16, _, _>(process_callback, consumer),
            SampleFormat::U16 => self.build_output_stream::<u16, _, _>(process_callback, consumer),
            format => return Err(format!("Unsupported output sample format: {format:?}")),
        }?;

        Ok(())
    }

    fn play(&self) -> Result<(), String> {
        let input_stream = self
            .input_stream
            .as_ref()
            .ok_or("No input stream configurated")?;

        let output_stream = self
            .output_stream
            .as_ref()
            .ok_or("No output stream configurated")?;

        input_stream.play().map_err(|e| e.to_string())?;
        output_stream.play().map_err(|e| e.to_string())?;

        println!("Playing audio streams");

        Ok(())
    }

    fn build_input_stream<T, P>(
        &mut self,
        // mut process_callback: C,
        mut producer: P,
    ) -> Result<(), String>
    where
        T: cpal::Sample + cpal::SizedSample,
        f32: cpal::FromSample<T>,
        P: Producer<Item = f32> + Send + 'static,
    {
        let input = self
            .device_manager
            .active_input_device
            .as_ref()
            .ok_or("Inpud device is not selected")?;

        let config = self
            .config
            .as_ref()
            .ok_or("Input config is not resolved")?
            .input
            .stream_config
            .clone();

        let input_stream = input
            .build_input_stream(
                &config,
                move |data: &[T], _| {
                    for frame in data.chunks(config.channels as usize) {
                        let mono = if config.channels >= 2 {
                            (frame[0].to_sample::<f32>() + frame[1].to_sample::<f32>()) * 0.5
                        } else {
                            frame[0].to_sample::<f32>()
                        };

                        // let processed = process_callback(raw);
                        _ = producer.try_push(mono);
                    }
                },
                move |err| eprintln!("Input error: {err}"),
                None,
            )
            .map_err(|e| e.to_string())?;

        self.input_stream = Some(input_stream);

        Ok(())
    }

    fn build_output_stream<T, P, C>(
        &mut self,
        mut block_processor: P,
        mut consumer: C,
    ) -> Result<(), String>
    where
        C: Consumer<Item = f32> + Send + 'static,
        T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32>,
        P: FnMut(&mut [f32]) + Send + 'static,
    {
        let output = self
            .device_manager
            .active_output_device
            .as_ref()
            .ok_or("Inpud device is not selected")?;

        let config = self
            .config
            .as_ref()
            .ok_or("Input config is not resolved")?
            .output
            .stream_config
            .clone();
        let channels = config.channels as usize;
        let mut scratch_buffer = Vec::new();

        let output_stream = output
            .build_output_stream(
                &config,
                move |data: &mut [T], _| {
                    let frame_count = data.len() / channels;

                    if scratch_buffer.len() != frame_count {
                        println!("{}", frame_count);
                        scratch_buffer.resize(frame_count, 0.0);
                    }

                    for sample in scratch_buffer.iter_mut() {
                        *sample = consumer.try_pop().unwrap_or(0.0);
                    }

                    block_processor(&mut scratch_buffer);

                    let mut sample_idx = 0;
                    for frame in data.chunks_mut(channels) {
                        if sample_idx >= scratch_buffer.len() {
                            break;
                        }
                        let processed_sample = scratch_buffer[sample_idx];

                        for out in frame {
                            *out = T::from_sample(processed_sample);
                        }
                        sample_idx += 1;
                    }
                },
                move |err| eprintln!("Output error: {err}"),
                None,
            )
            .map_err(|e| e.to_string())?;

        self.output_stream = Some(output_stream);
        Ok(())
    }

    pub fn resolve_configs(&mut self, preffered_rates: Vec<u32>) -> Result<(), String> {
        let input = self
            .device_manager
            .active_input_device
            .as_ref()
            .ok_or("Inpud device is not selected")?;

        let output = self
            .device_manager
            .active_output_device
            .as_ref()
            .ok_or("Output device is not selected")?;

        let input_configs: Vec<_> = input
            .supported_input_configs()
            .map_err(|err| err.to_string())?
            .collect();

        let output_configs: Vec<_> = output
            .supported_output_configs()
            .map_err(|err| err.to_string())?
            .collect();

        for rate in preffered_rates {
            let sr: SampleRate = rate;

            let input_match = input_configs
                .iter()
                .find(|c| c.min_sample_rate() <= sr && c.max_sample_rate() >= sr);

            let output_match = output_configs
                .iter()
                .find(|c| c.min_sample_rate() <= sr && c.max_sample_rate() >= sr);

            if let (Some(input_cfg), Some(output_cfg)) = (input_match, output_match) {
                let input_supported = input_cfg.with_sample_rate(sr);
                let output_supported = output_cfg.with_sample_rate(sr);

                let resolved_config = ResolvedAudioConfig {
                    input: ResolvedStreamConfig {
                        stream_config: input_supported.config(),
                        sample_format: input_supported.sample_format(),
                    },
                    output: ResolvedStreamConfig {
                        stream_config: output_supported.config(),
                        sample_format: output_supported.sample_format(),
                    },
                    sample_rate: sr,
                };

                self.config = Some(resolved_config);

                return Ok(());
            }
        }

        Err("No compatible input/output sample rate found".to_string())
    }
}
