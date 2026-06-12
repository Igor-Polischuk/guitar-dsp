use crate::wdf::wdf_node::WDFNode;

pub struct Capacitor {
    pub capacitance_fr: f32,
    pub sample_rate: f32,
    pub r: f32,
    pub state: f32,
}

impl Capacitor {
    pub fn new(capacitance_fr: f32, sample_rate: f32) -> Self {
        let mut cap = Self {
            capacitance_fr,
            sample_rate,
            r: 0.0,
            state: 0.0,
        };
        cap.update_impedance();
        cap
    }
    //pf
    pub fn from_picofarads(v: f32, sample_rate: f32) -> Self {
        let capacity_fr = v * 0.000000000001;
        Capacitor::new(capacity_fr, sample_rate)
    }

    //uF
    pub fn from_microfarad(v: f32, sample_rate: f32) -> Self {
        let capacity_fr = v * 0.000001;
        Capacitor::new(capacity_fr, sample_rate)
    }
}

impl WDFNode for Capacitor {
    fn update_impedance(&mut self) {
        // R = T / (2 * C)
        self.r = 1.0 / (2.0 * self.sample_rate * self.capacitance_fr);
    }

    fn get_impedance(&self) -> f32 {
        self.r
    }

    #[inline(always)]
    fn propagate_up(&mut self) -> f32 {
        self.state
    }

    #[inline(always)]
    fn propagate_down(&mut self, a: f32) {
        self.state = a;
    }
}

//0.022uF = 0.000000022F
//470pf =   0.00000000047F
