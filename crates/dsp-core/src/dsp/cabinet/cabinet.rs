use crate::dsp::cabinet::{
    convolution::Convolution,
    helpers::{load_embedded_ir, peak_normalize, transform_ir, trim_ir_start},
};

pub enum Cabinet {
    CenzoCelestion,
    OpenBack,
    MarshalGreenback,
    Marshal4x12V30,
}

pub struct CabinetManager<const N: usize> {
    sample_rate: f32,
}

impl<const N: usize> CabinetManager<N> {
    pub fn new(sample_rate: f32) -> Self {
        CabinetManager { sample_rate }
    }

    pub fn get_cabinet(&self, cab: Cabinet) -> Convolution<N> {
        let raw_ir = self.get_cabinet_ir(cab);
        let ir = self.process_ir(raw_ir);

        Convolution::new(ir)
    }

    fn get_cabinet_ir(&self, cab: Cabinet) -> &[u8] {
        match cab {
            Cabinet::CenzoCelestion => {
                include_bytes!("../../assets/impulse_responsers/CenzoCelestion.wav")
            }
            Cabinet::OpenBack => include_bytes!(
                // "../../assets/impulse_responsers/OpenBack2x12/Ribbon/Open-Back 2x12 with Goodmans Green Label Mid 60s (Ribbon - Center).wav"
                "../../assets/impulse_responsers/OpenBack2x12/AKD130/Open-Back 2x12 with Goodmans Green Label Mid 60s (AKG D130 - Back).wav"
            ),
            Cabinet::MarshalGreenback => include_bytes!(
                "../../assets/impulse_responsers/GreenbackMarshal/M25 LL 1960TV 4x12 SM57 1.50in 0.0in SA73.wav"
            ),
            Cabinet::Marshal4x12V30 => {
                include_bytes!("../../assets/impulse_responsers/Marshal4x12V30/EV MIX D.wav")
            }
        }
    }

    fn process_ir(&self, cab_ir_bytes: &[u8]) -> [f32; N] {
        let ir = load_embedded_ir(cab_ir_bytes, self.sample_rate).unwrap();
        let ir = trim_ir_start(ir);
        let ir = peak_normalize(ir);
        let ir = transform_ir(ir);

        ir
    }
}
