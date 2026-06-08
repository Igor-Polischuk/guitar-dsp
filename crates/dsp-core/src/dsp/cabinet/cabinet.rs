use crate::dsp::cabinet::{
    convolution::Convolution,
    helpers::{
        apply_gain, fade_out_tail, fit_to_length, load_embedded_response, normalize_peak,
        remove_dc_offset, trim_leading_silence,
    },
};

pub enum Cabinet {
    CenzoCelestion,
    OpenBack2x12,
    MarshallGreenback4x12,
    MarshallV30_4x12,
}

pub struct CabinetFactory {
    sample_rate: f32,
}

impl CabinetFactory {
    pub fn new(sample_rate: f32) -> Self {
        CabinetFactory { sample_rate }
    }

    pub fn create_cabinet(&self, cab: Cabinet) -> Convolution<8192> {
        let raw_ir = self.load_cabinet_asset(cab);
        let ir = self.prepare_cabinet_response(raw_ir);

        Convolution::new(ir)
    }

    fn load_cabinet_asset(&self, cab: Cabinet) -> &[u8] {
        match cab {
            Cabinet::CenzoCelestion => {
                include_bytes!("../../assets/impulse_responsers/CenzoCelestion.wav")
            }
            Cabinet::OpenBack2x12 => include_bytes!(
                // "../../assets/impulse_responsers/OpenBack2x12/Ribbon/Open-Back 2x12 with Goodmans Green Label Mid 60s (Ribbon - Center).wav"
                "../../assets/impulse_responsers/OpenBack2x12/AKD130/Open-Back 2x12 with Goodmans Green Label Mid 60s (AKG D130 - Back).wav"
            ),
            Cabinet::MarshallGreenback4x12 => include_bytes!(
                "../../assets/impulse_responsers/GreenbackMarshal/M25 LL 1960TV 4x12 SM57 1.50in 0.0in SA73.wav"
            ),
            Cabinet::MarshallV30_4x12 => {
                include_bytes!("../../assets/impulse_responsers/Marshal4x12V30/EV MIX D.wav")
            }
        }
    }

    fn prepare_cabinet_response(&self, asset_bytes: &[u8]) -> [f32; 8192] {
        let response = load_embedded_response(asset_bytes, self.sample_rate)
            .expect("Failed to load cabinet response");

        let response = remove_dc_offset(response);

        let response = trim_leading_silence(response);
        let response = normalize_peak(response);
        let response = apply_gain(response, 0.2);

        let mut response = fit_to_length::<8192>(response);
        fade_out_tail(&mut response, 512);

        response
    }
}
