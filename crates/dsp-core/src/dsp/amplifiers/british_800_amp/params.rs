use std::sync::Arc;

use crate::utils::AtomicF32;

pub struct British800Params {
    pub presence: Arc<AtomicF32>,
    pub bass: Arc<AtomicF32>,
    pub mid: Arc<AtomicF32>,
    pub treble: Arc<AtomicF32>,
    pub master_volume: Arc<AtomicF32>,
    pub pre_amp_volume: Arc<AtomicF32>,
}

impl Default for British800Params {
    fn default() -> Self {
        British800Params {
            presence: Arc::new(AtomicF32::new(0.0)),
            bass: Arc::new(AtomicF32::new(0.0)),
            mid: Arc::new(AtomicF32::new(0.0)),
            treble: Arc::new(AtomicF32::new(0.0)),
            master_volume: Arc::new(AtomicF32::new(0.0)),
            pre_amp_volume: Arc::new(AtomicF32::new(0.0)),
        }
    }
}
