use crate::wdf::wdf_node::WDFNode;

pub struct ParalelAdaptor {
    port1: Box<dyn WDFNode>,
    port2: Box<dyn WDFNode>,
    r: f32,
    gamma: f32,
    saved_a1: f32,
    saved_a2: f32,
}

impl ParalelAdaptor {
    pub fn new(port1: Box<dyn WDFNode>, port2: Box<dyn WDFNode>) -> Self {
        let r1 = port1.get_impedance();
        let r2 = port2.get_impedance();
        let r = (r1 * r2) / (r1 + r2);
        let gamma = r2 / (r1 + r2);
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

impl WDFNode for ParalelAdaptor {
    fn get_impedance(&self) -> f32 {
        self.r
    }

    fn propagate_down(&mut self, a: f32) {
        let v = self.gamma * (self.saved_a1 - self.saved_a2) + self.saved_a2 + a;

        let b1 = 2.0 * v - self.saved_a1;
        let b2 = 2.0 * v - self.saved_a2;

        self.port1.propagate_down(b1);
        self.port2.propagate_down(b2);
    }

    fn propagate_up(&mut self) -> f32 {
        self.saved_a1 = self.port1.propagate_up();
        self.saved_a2 = self.port2.propagate_up();

        // self.gamma * self.saved_a1 + (1.0 - self.gamma) * self.saved_a2
        (1.0 - self.gamma) * self.saved_a1 + self.gamma * self.saved_a2
    }

    fn update_impedance(&mut self) {
        self.port1.update_impedance();
        self.port2.update_impedance();
        let r1 = self.port1.get_impedance();
        let r2 = self.port2.get_impedance();
        self.r = (r1 * r2) / (r1 + r2);
        self.gamma = r2 / (r1 + r2);
    }
}
