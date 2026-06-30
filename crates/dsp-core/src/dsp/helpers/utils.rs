#[inline]
pub fn db_to_gain(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

#[inline]
pub fn tube_stage(x: f32, drive: f32, bias: f32, out_gain: f32) -> f32 {
    let y = (x * drive + bias).tanh() - bias.tanh();
    y * out_gain
}

#[inline]
pub fn tube_stage_unity(x: f32, drive: f32, bias: f32, mix: f32) -> f32 {
    let bias_tanh = bias.tanh();

    // Small-signal slope of tanh(x * drive + bias) around x=0
    let small_signal_gain = drive * (1.0 - bias_tanh * bias_tanh);

    let y = ((x * drive + bias).tanh() - bias_tanh) / small_signal_gain.max(1e-6);

    x * (1.0 - mix) + y * mix
}
