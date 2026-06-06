use crate::dsp::cabinet::{
    convolution::Convolution,
    helpers::{load_embedded_ir, peak_normalize, transform_ir},
};

pub enum Cabinet {
    CenzoCelestion,
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
        }
    }

    fn process_ir(&self, cab_ir_bytes: &[u8]) -> [f32; N] {
        let ir = load_embedded_ir(cab_ir_bytes, self.sample_rate).unwrap();
        let ir = peak_normalize(ir);
        let ir = transform_ir(ir);

        ir
    }
}
