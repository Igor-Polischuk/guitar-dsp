use std::sync::Arc;

use crate::utils::AtomicF32;

/// Absolute values of params for specific AMP
pub struct AMPParms {
    pub id: &'static str,
    pub label: &'static str,
    pub value: Arc<AtomicF32>,
    pub min: f32,
    pub max: f32,
    pub default: f32,
}

#[derive(Clone, serde::Serialize)]
pub struct KnobDescriptor {
    pub id: &'static str,
    pub label: &'static str,
    pub min: f32,
    pub max: f32,
    pub default: f32,
    pub step: f32,
    pub unit: Option<&'static str>,
}

#[derive(Clone, serde::Serialize)]
pub struct InputDescriptor {
    pub id: &'static str,
    pub label: &'static str,
}
