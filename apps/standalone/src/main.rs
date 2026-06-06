use audio_io::prelude::*;
use dsp_core::dsp::{
    Cabinet, CabinetManager, Distortion, DistortionPreset, Equalizer, Gain, HighPassFilter,
    LowPassFilter, MasterVolume, SignalChain,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut audio = AudioIO::init();

    audio.start(
        AudioIoSettings {
            input_device_name: Some("Volt 1".into()),
            output_device_name: Some("MacBook Pro Speakers".into()),
            ..Default::default()
        },
        move |ctx| {
            let mut processing_chain = build_chain(ctx.sample_rate as f32);
            move |sample| processing_chain.process(sample)
        },
    )?;

    println!("Listening... Press Enter to stop.");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}

fn build_chain(sample_rate: f32) -> SignalChain {
    let mut processing_chain = SignalChain::new(sample_rate);
    let gain = Gain::new(8).unwrap();
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
    processing_chain.append_node(distortion);
    processing_chain.append_node(eq);
    processing_chain.append_node(cabinet);
    processing_chain.append_node(low_pass_filter);
    processing_chain.append_node(volume);

    processing_chain
}
