use crate::dsp::amplifiers::amp_param::{InputDescriptor, KnobDescriptor};

pub const BRITISH_800_KNOBS: &[KnobDescriptor] = &[
    KnobDescriptor {
        id: "presence",
        label: "Presence",
        min: 0.0,
        max: 1.0,
        default: 0.5,
        unit: None,
        step: 0.1,
    },
    KnobDescriptor {
        id: "bass",
        label: "Bass",
        min: 0.0,
        max: 10.0,
        default: 5.0,
        unit: None,
        step: 1.0,
    },
    KnobDescriptor {
        id: "mid",
        label: "Middle",
        min: 0.0,
        max: 10.0,
        default: 5.0,
        unit: None,
        step: 1.0,
    },
    KnobDescriptor {
        id: "treble",
        label: "Treble",
        min: 0.0,
        max: 10.0,
        default: 5.0,
        unit: None,
        step: 1.0,
    },
    KnobDescriptor {
        id: "master_volume",
        label: "Master",
        min: 0.0,
        max: 10.0,
        default: 10.0,
        unit: None,
        step: 1.0,
    },
    KnobDescriptor {
        id: "pre_amp_volume",
        label: "Pre amp",
        min: 0.0,
        max: 10.0,
        default: 5.0,
        unit: None,
        step: 1.0,
    },
];

pub const BRITISH_800_INPUTS: &[InputDescriptor] = &[
    InputDescriptor {
        id: "high_input",
        label: "High",
    },
    InputDescriptor {
        id: "low_input",
        label: "Low",
    },
];
