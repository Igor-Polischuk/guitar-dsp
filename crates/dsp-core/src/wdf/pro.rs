use crate::wdf::{
    capacitor::Capacitor, parallel_adapter::ParalelAdaptor, resistor::Resistor,
    serial_adapter::SerialAdaptor, wdf_node::WDFNode,
};

pub struct ToneStack {
    root: Box<dyn WDFNode>,
}

impl ToneStack {
    pub fn new(sample_rate: f32) -> Self {
        let treble_p = 0.99;
        let mid_p = 0.99999;
        let bas_p = 0.99999;

        // Компоненти (номінали залишаємо твої, вони правильні)
        let slope_t = Resistor::new(33000.0); // у Маршалла зазвичай 33k
        let treble_r = Resistor::new(250000.0 * treble_p);
        let mid_r = Resistor::new(22000.0 * mid_p);
        let bass_r = Resistor::new(1000000.0 * bas_p);

        let treble_c = Capacitor::from_picofarads(470.0, sample_rate);
        let mid_c = Capacitor::from_microfarad(0.022, sample_rate);
        let bass_c = Capacitor::from_microfarad(0.022, sample_rate);

        // --- ПЕРЕЗБИРАЄМО З'ЄДНАННЯ ---

        // 1. Секція Bass: Конденсатор паралельно резистору
        let bass_section = ParalelAdaptor::new(Box::new(bass_c), Box::new(bass_r));

        // 2. Секція Mid: Конденсатор послідовно з резистором
        let mid_section = SerialAdaptor::new(Box::new(mid_c), Box::new(mid_r));

        // 3. Об'єднуємо Bass та Mid послідовно
        let bass_mid_line = SerialAdaptor::new(Box::new(bass_section), Box::new(mid_section));

        // 4. Підключаємо Slope резистор послідовно до нижньої лінії
        let bottom_line = SerialAdaptor::new(Box::new(slope_t), Box::new(bass_mid_line));

        // 5. Секція Treble: Конденсатор послідовно з потенціометром Treble
        let treble_line = SerialAdaptor::new(Box::new(treble_c), Box::new(treble_r));

        // 6. Корінь системи: Паралельно об'єднуємо гілку високих та нижню лінію
        let mut root = ParalelAdaptor::new(Box::new(treble_line), Box::new(bottom_line));

        root.update_impedance();

        Self {
            root: Box::new(root),
        }
    }

    pub fn process_sample(&mut self, sample: f32) -> f32 {
        // 1. Отримуємо опір усього нашого темброблоку в цій точці
        let r_root = self.root.get_impedance();

        // Внутрішній вихідний опір катодного повторювача Маршалла (близько 1 кОм)
        let r_g = 1000.0;

        // 2. Фаза UP: збираємо хвилю знизу
        let b_root = self.root.propagate_up();

        // 3. Розрахунок падаючої хвилі з урахуванням реального джерела живлення
        let denominator = r_root + r_g;
        let term1 = (2.0 * r_root * sample) / denominator;
        let term2 = ((r_g - r_root) / denominator) * b_root;
        let a_root = term1 + term2;

        // 4. Фаза DOWN: проштовхуємо хвилю в дерево
        self.root.propagate_down(a_root);

        // 5. Напруга на вході темброблоку (для контролю)
        let input_stage_voltage = (a_root + b_root) / 2.0;

        // Знімаємо вихідний сигнал. Як ми обговорювали, у Маршалла вихід
        // знімається з повзунка Treble, але для перевірки працездатності дерева
        // тимчасово повернемо input_stage_voltage, щоб переконатися, що NaN зник!
        // println!("{input_stage_voltage}");
        let output_voltage = b_root * 0.99;
        if output_voltage.is_nan() {
            panic!("NaN detected!");
        }

        output_voltage
    }
}
