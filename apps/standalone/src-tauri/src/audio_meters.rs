// -60dbsd - 0 dbfs

use crate::state::EngineState;

pub fn block_peak(samples: &[f32]) -> f32 {
    samples.iter().map(|x| x.abs()).fold(0.0, f32::max)
}

pub fn peack_to_dbfs(peak: f32) -> f32 {
    let safe_peak = peak.max(1.0e-9);
    20.0 * safe_peak.log10()
}

// let rms = (samples.iter()
//     .map(|x| x*x)
//     .sum::<f32>() / samples.len() as f32)
//     .sqrt();

fn dbfs_to_meter_percent(db: f32) -> f32 {
    ((db + 60.0) / 60.0).clamp(0.0, 1.0)
}

#[derive(serde::Serialize)]
pub struct AudioMeterOutput {
    pub input_db: f32,
    pub output_db: f32,
}

#[tauri::command]
pub fn get_peak_dbs(state: tauri::State<EngineState>) -> AudioMeterOutput {
    let meter = &state.audio_meters;

    AudioMeterOutput {
        input_db: meter.input_peak_dbfs.get(),
        output_db: meter.output_peak_dbfs.get(),
    }
}
