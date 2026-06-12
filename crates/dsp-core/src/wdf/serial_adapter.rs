use crate::wdf::wdf_node::WDFNode;

pub struct SerialAdaptor {
    port1: Box<dyn WDFNode>,
    port2: Box<dyn WDFNode>,
    r: f32,
    gamma: f32,
    saved_a1: f32,
    saved_a2: f32,
}

impl SerialAdaptor {
    pub fn new(port1: Box<dyn WDFNode>, port2: Box<dyn WDFNode>) -> Self {
        let r1 = port1.get_impedance();
        let r2 = port2.get_impedance();
        let r = r1 + r2;
        let gamma = r1 / r;
        Self {
            port1,
            port2,
            r,
            gamma,
            saved_a1: 0.0,
            saved_a2: 0.0,
        }
    }
}

impl WDFNode for SerialAdaptor {
    fn get_impedance(&self) -> f32 {
        self.r
    }

    fn propagate_down(&mut self, a: f32) {
        let common_factor = a + self.saved_a1 + self.saved_a2;
        // let common_factor = self.saved_a1 + self.saved_a2 - a;

        let b1 = self.saved_a1 - self.gamma * common_factor;
        let b2 = self.saved_a2 - (1.0 - self.gamma) * common_factor;

        self.port1.propagate_down(b1);
        self.port2.propagate_down(b2);
    }

    fn propagate_up(&mut self) -> f32 {
        self.saved_a1 = self.port1.propagate_up();
        self.saved_a2 = self.port2.propagate_up();

        -(self.saved_a1 + self.saved_a2)
    }

    fn update_impedance(&mut self) {
        self.port1.update_impedance();
        self.port2.update_impedance();
        let r1 = self.port1.get_impedance();
        let r2 = self.port2.get_impedance();
        self.r = r1 + r2;
        self.gamma = r1 / self.r;
    }
}
