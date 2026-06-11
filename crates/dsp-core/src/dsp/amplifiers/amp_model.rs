use crate::dsp::British800Params;

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum AmpModel {
    British800,
}

pub enum ActiveAmpParams {
    British800(British800Params),
}
