pub trait WDFNode: Send + Sync {
    fn propagate_up(&mut self) -> f32;
    fn propagate_down(&mut self, a: f32);
    /// To update resistence
    fn update_impedance(&mut self);
    fn get_impedance(&self) -> f32;
}
