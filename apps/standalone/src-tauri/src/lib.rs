mod audio_meters;
mod devices_managment;
mod state;

use devices_managment::{is_devices_selected, list_audio_devices, set_devices};
use state::{ActiveAmpState, EngineState};

use audio_io::prelude::*;
use dsp_core::dsp::{ActiveAmpParams, AmpModel, AmpNode, InputDescriptor, KnobDescriptor};
use dsp_core::prelude::*;
use std::default::Default;
use std::sync::{Arc, Mutex};
use tauri::Manager;

use crate::audio_meters::{block_peak, get_peak_dbs, peack_to_dbfs};
use crate::state::AudioMeters;

#[tauri::command]
fn update_amp_parameter(parameter_id: &str, value: f32, engine: tauri::State<EngineState>) {
    let active_amp = engine.active_amp.lock().unwrap();

    match &active_amp.params {
        ActiveAmpParams::British800(params) => {
            params.set(parameter_id, value);
        }
    }
}

#[tauri::command]
fn get_current_amplifier_knobs(engine: tauri::State<EngineState>) -> Vec<KnobDescriptor> {
    let selected = *&engine.active_amp.lock().unwrap().model;

    match selected {
        AmpModel::British800 => British800Amp::knobs().to_vec(),
    }
}

#[tauri::command]
fn get_current_amplifier_inputs(engine: tauri::State<EngineState>) -> Vec<InputDescriptor> {
    let selected_amp = *&engine.active_amp.lock().unwrap().model;

    match selected_amp {
        AmpModel::British800 => British800Amp::inputs().to_vec(),
    }
}

#[tauri::command]
fn get_current_amplifier_active_input(engine: tauri::State<EngineState>) -> String {
    let active_amp = engine.active_amp.lock().unwrap();

    match &active_amp.params {
        ActiveAmpParams::British800(params) => params.active_input_id().to_string(),
    }
}

#[tauri::command]
fn set_active_amp_input(input_id: &str, engine: tauri::State<EngineState>) -> Result<(), String> {
    let active_amp = engine.active_amp.lock().unwrap();

    match &active_amp.params {
        ActiveAmpParams::British800(params) => params.set_input(input_id),
    }
}

#[tauri::command]
fn set_active_amp(model: AmpModel, engine: tauri::State<EngineState>) {
    let params = match model {
        AmpModel::British800 => ActiveAmpParams::British800(British800Params::default()),
    };

    *engine.active_amp.lock().unwrap() = ActiveAmpState { model, params };
}

fn build_chain(sample_rate: f32, engine: tauri::State<EngineState>) -> SignalChain {
    let mut processing_chain = SignalChain::new();
    let mut pre_cab = SampleProcessingChain::new();

    {
        let active_amp = engine.active_amp.lock().unwrap();

        match &active_amp.params {
            ActiveAmpParams::British800(params) => {
                pre_cab.append_node(British800Amp::new(sample_rate, &params.clone()));
            }
        }
    }

    // let cabinet_factory = CabinetFactory::new(sample_rate);
    // let cab = cabinet_factory.create_cabinet(Cabinet::MarshallV30_4x12);
    processing_chain.append_node(pre_cab);
    // processing_chain.append_node(cab);

    processing_chain
}

#[tauri::command]
fn start_audio(
    state: tauri::State<EngineState>,
    current_amplifier: tauri::State<EngineState>,
) -> Result<(), String> {
    let mut audio = state.audio.lock().unwrap();
    let input_peak_dbfs = state.audio_meters.input_peak_dbfs.clone();
    let output_peak_dbfs = state.audio_meters.output_peak_dbfs.clone();

    if audio.active_input().is_none() || audio.active_output().is_none() {
        return Err(String::from("Can't start audio without devices"));
    }
    audio.start(AudioIoSettings::default(), move |ctx| {
        let mut processing_chain = build_chain(ctx.sample_rate as f32, current_amplifier);
        move |samples_block| {
            let input_peak = block_peak(&samples_block);
            let peak_db = peack_to_dbfs(input_peak);
            input_peak_dbfs.set(peak_db);
            processing_chain.process(samples_block);
            let output_peak = block_peak(&samples_block);
            let peak_db = peack_to_dbfs(output_peak);
            output_peak_dbfs.set(peak_db);
        }
    })?;

    Ok(())
}

#[tauri::command]
fn stop_audio(state: tauri::State<EngineState>) {
    state.audio.lock().unwrap().stop();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let stat = AudioStat::new();
            let audio = AudioIO::init(stat);
            app.manage(EngineState {
                audio: Mutex::new(audio),
                active_amp: {
                    Mutex::new(ActiveAmpState {
                        model: AmpModel::British800,
                        params: ActiveAmpParams::British800(British800Params::default()),
                    })
                },
                audio_meters: AudioMeters {
                    input_peak_dbfs: Arc::new(AtomicF32::new(-100.0)),
                    output_peak_dbfs: Arc::new(AtomicF32::new(-100.0)),
                },
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_audio,
            stop_audio,
            update_amp_parameter,
            set_active_amp,
            get_current_amplifier_knobs,
            get_current_amplifier_inputs,
            get_current_amplifier_active_input,
            set_active_amp_input,
            list_audio_devices,
            is_devices_selected,
            set_devices,
            get_peak_dbs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
