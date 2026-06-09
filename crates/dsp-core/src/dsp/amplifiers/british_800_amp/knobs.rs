use crate::dsp::amplifiers::amp_param::KnobDescriptor;

pub const BRITISH_800_KNOBS: &[KnobDescriptor] = &[
    KnobDescriptor {
        id: "presence",
        label: "Presence",
        min: 0.0,
        max: 10.0,
        default: 5.0,
        unit: None,
    },
    KnobDescriptor {
        id: "bass",
        label: "Bass",
        min: 0.0,
        max: 10.0,
        default: 5.0,
        unit: None,
    },
    KnobDescriptor {
        id: "mid",
        label: "Middle",
        min: 0.0,
        max: 10.0,
        default: 6.0,
        unit: None,
    },
    KnobDescriptor {
        id: "treble",
        label: "Treble",
        min: 0.0,
        max: 10.0,
        default: 5.0,
        unit: None,
    },
    KnobDescriptor {
        id: "master_volume",
        label: "Master",
        min: 0.0,
        max: 10.0,
        default: 4.0,
        unit: None,
    },
    KnobDescriptor {
        id: "pre_amp_volume",
        label: "Pre amp",
        min: 0.0,
        max: 10.0,
        default: 4.0,
        unit: None,
    },
];
