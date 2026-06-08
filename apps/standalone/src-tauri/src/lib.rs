use audio_io::prelude::*;
use dsp_core::prelude::*;
use std::default::Default;
use std::sync::{Arc, Mutex};
use tauri::Manager;

struct AudioState {
    audio: Mutex<Option<AudioIO>>,
}

struct AmpParams {
    gain: Arc<AtomicF32>,
    equalizer_params: Arc<EqualizerParams>,
    lpf_cutoff_hz: Arc<AtomicF32>,
    hpf_cutoff_hz: Arc<AtomicF32>,
    volume: Arc<AtomicF32>,
}

#[tauri::command]
fn update_parameter(label: &str, value: f32, amp: tauri::State<AmpParams>) {
    match label {
        "GAIN" => amp.gain.set(value),
        "BASS" => amp.equalizer_params.set_bass(value),
        "MID" => amp.equalizer_params.set_mid(value),
        "TREBLE" => amp.equalizer_params.set_treble(value),
        "LPF" => amp.lpf_cutoff_hz.set(value),
        "HPF" => amp.hpf_cutoff_hz.set(value),
        "MASTER" => amp.volume.set(value),
        _ => {}
    }
}

fn build_chain(sample_rate: f32, amp: tauri::State<AmpParams>) -> SignalChain {
    let mut processing_chain = SignalChain::new();
    let gain = Gain::new(amp.gain.clone());
    let eq = Equalizer::new(sample_rate, amp.equalizer_params.clone());
    let lpf = LowPassFilter::new(amp.lpf_cutoff_hz.clone(), sample_rate);
    let hpf = HighPassFilter::new(amp.hpf_cutoff_hz.clone(), sample_rate);
    let volume = MasterVolume::new(sample_rate, amp.volume.clone());
    let dist = Distortion::new(DistortionPreset::SmoothTube);

    let mut pre_cab = SampleProcessingChain::new();

    let cabinet_manager = CabinetFactory::new(sample_rate);
    let cab = cabinet_manager.create_cabinet(Cabinet::MarshallV30_4x12);
    pre_cab.append_node(hpf);
    pre_cab.append_node(gain);
    pre_cab.append_node(dist);
    pre_cab.append_node(eq);
    pre_cab.append_node(lpf);
    processing_chain.append_node(pre_cab);
    processing_chain.append_node(cab);
    let mut after_cab = SampleProcessingChain::new();
    after_cab.append_node(volume);
    processing_chain.append_node(after_cab);

    processing_chain
}

#[tauri::command]
fn start_audio(
    state: tauri::State<AudioState>,
    amp: tauri::State<AmpParams>,
) -> Result<(), String> {
    let mut audio = AudioIO::init();

    audio.start(
        AudioIoSettings {
            input_device_name: Some("Volt 1".into()),
            output_device_name: Some("MacBook Pro Speakers".into()),
            ..Default::default()
        },
        move |ctx| {
            let mut processing_chain = build_chain(ctx.sample_rate as f32, amp);
            move |samples_block| processing_chain.process(samples_block)
        },
    )?;

    *state.audio.lock().unwrap() = Some(audio);
    Ok(())
}

#[tauri::command]
fn stop_audio(state: tauri::State<AudioState>) {
    *state.audio.lock().unwrap() = None;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AudioState {
                audio: Mutex::new(None),
            });
            app.manage(AmpParams {
                gain: Arc::new(AtomicF32::new(10.0)),
                equalizer_params: Arc::new(EqualizerParams {
                    ..Default::default()
                }),
                lpf_cutoff_hz: Arc::new(AtomicF32::new(8000.0)),
                hpf_cutoff_hz: Arc::new(AtomicF32::new(80.0)),
                volume: Arc::new(AtomicF32::new(0.0)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            update_parameter,
            start_audio,
            stop_audio
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
