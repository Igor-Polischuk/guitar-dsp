use audio_io::prelude::AudioIoDevice;

use crate::state::EngineState;

#[tauri::command]
pub fn list_audio_devices(engine: tauri::State<EngineState>) -> Vec<AudioIoDevice> {
    let audio_io = engine.audio.lock().unwrap();

    if let Ok(devices) = audio_io.available_devices() {
        return devices;
    }

    vec![]
}

#[tauri::command]
pub fn is_devices_selected(state: tauri::State<EngineState>) -> bool {
    let audio = state.audio.lock().unwrap();

    let selected_input = audio.active_input().is_none();
    let selected_output = audio.active_output().is_none();

    return !(selected_input || selected_output);
}

#[tauri::command]
pub fn set_devices(
    input: &str,
    output: &str,
    state: tauri::State<EngineState>,
) -> Result<(), String> {
    let mut audio = state.audio.lock().unwrap();

    audio.set_input_device(input)?;
    audio.set_output_device(output)?;

    Ok(())
}
