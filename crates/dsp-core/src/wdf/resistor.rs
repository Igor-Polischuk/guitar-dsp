use crate::wdf::wdf_node::WDFNode;

pub struct Resistor {
    pub r: f32,
}

impl Resistor {
    pub fn new(r: f32) -> Self {
        Self { r }
    }
}

impl WDFNode for Resistor {
    fn update_impedance(&mut self) {}

    fn get_impedance(&self) -> f32 {
        self.r
    }

    #[inline(always)]
    fn propagate_up(&mut self) -> f32 {
        0.0
    }

    #[inline(always)]
    fn propagate_down(&mut self, _a: f32) {}
}
