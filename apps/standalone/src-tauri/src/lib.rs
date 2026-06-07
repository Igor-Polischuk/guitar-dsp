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
    equilizer_params: Arc<EqualizerParams>,
}

#[tauri::command]
fn update_parameter(label: &str, value: f32, amp: tauri::State<AmpParams>) {
    if label == "GAIN" {
        amp.gain.set(value);
    }

    if label == "BASS" {
        amp.equilizer_params.set_bass(value);
    }

    if label == "MID" {
        amp.equilizer_params.set_mid(value);
    }

    if label == "TREBLE" {
        amp.equilizer_params.set_treble(value);
    }
}

fn build_chain(sample_rate: f32, amp: tauri::State<AmpParams>) -> SignalChain {
    let mut processing_chain = SignalChain::new(sample_rate);
    let gain = Gain::new(amp.gain.clone());
    let eq = Equalizer::new(sample_rate, amp.equilizer_params.clone());

    processing_chain.append_node(gain);
    processing_chain.append_node(eq);

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
            move |sample| processing_chain.process(sample)
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
                equilizer_params: Arc::new(EqualizerParams {
                    ..Default::default()
                }),
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
