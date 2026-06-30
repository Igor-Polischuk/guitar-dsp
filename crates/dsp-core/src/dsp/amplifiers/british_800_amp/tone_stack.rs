use std::sync::Arc;

use crate::utils::AtomicF32;

#[derive(Copy, Clone)]
enum Node {
    A = 0,
    B = 1,
    T = 2,
    C = 3,
    M = 4,
    Out = 5,
}

const N: usize = 6;
const GND: Option<usize> = None;

pub struct NodalCapacitor {
    n1: Option<usize>,
    n2: Option<usize>,
    // c: f32,
    g: f32,
    i_hist: f32,
}

impl NodalCapacitor {
    pub fn new(n1: Option<usize>, n2: Option<usize>, c: f32, sample_rate: f32) -> Self {
        let g = 2.0 * c * sample_rate;

        Self {
            n1,
            n2,
            // c,
            g,
            i_hist: 0.0,
        }
    }

    fn stamp(&self, g_matrix: &mut [[f32; N]; N], rhs: &mut [f32; N]) {
        stamp_conductance(g_matrix, self.n1, self.n2, self.g);

        if let Some(n1) = self.n1 {
            rhs[n1] -= self.i_hist;
        }

        if let Some(n2) = self.n2 {
            rhs[n2] += self.i_hist;
        }
    }

    fn update(&mut self, voltages: &[f32; N]) {
        let v1 = self.n1.map(|n| voltages[n]).unwrap_or(0.0);
        let v2 = self.n2.map(|n| voltages[n]).unwrap_or(0.0);

        let v = v1 - v2;

        let i = self.g * v + self.i_hist;

        self.i_hist = -self.g * v - i;
    }
}

pub struct MarshallToneStack {
    // sample_rate: f32,
    bass: Arc<AtomicF32>,
    mid: Arc<AtomicF32>,
    treble: Arc<AtomicF32>,

    c_treble: NodalCapacitor,
    c_bass: NodalCapacitor,
    c_mid: NodalCapacitor,
}

impl MarshallToneStack {
    pub fn new(
        sample_rate: f32,
        bass: Arc<AtomicF32>,
        mid: Arc<AtomicF32>,
        treble: Arc<AtomicF32>,
    ) -> Self {
        Self {
            // sample_rate,
            bass,
            mid,
            treble,

            // 470pF: A -> T
            c_treble: NodalCapacitor::new(
                Some(Node::A as usize),
                Some(Node::T as usize),
                470e-12,
                sample_rate,
            ),

            // 22nF: B -> C
            c_bass: NodalCapacitor::new(
                Some(Node::B as usize),
                Some(Node::C as usize),
                22e-9,
                sample_rate,
            ),

            // 22nF: B -> M
            c_mid: NodalCapacitor::new(
                Some(Node::B as usize),
                Some(Node::M as usize),
                22e-9,
                sample_rate,
            ),
        }
    }

    pub fn process_sample(&mut self, input: f32) -> f32 {
        let mut g = [[0.0_f32; N]; N];
        let mut rhs = [0.0_f32; N];

        // -------------------------
        // Constants
        // -------------------------

        let r_source = 1300.0; // cathode follower output impedance approximation
        let r_slope = 33_000.0;

        let r_treble_total = 250_000.0;
        let r_bass_total = 1_000_000.0;
        let r_mid_total = 22_000.0;

        let r_load = 1_000_000.0; // master volume / next stage load approximation

        let min_r = 10.0;

        // -------------------------
        // Input source: Vin -> Rsource -> Node A
        // -------------------------

        let g_src = 1.0 / r_source;
        g[Node::A as usize][Node::A as usize] += g_src;
        rhs[Node::A as usize] += g_src * input;

        // -------------------------
        // Fixed resistor
        // -------------------------

        stamp_resistor(
            &mut g,
            Some(Node::A as usize),
            Some(Node::B as usize),
            r_slope,
        );

        // -------------------------
        // Potentiometers
        // -------------------------

        let treble = self.treble.get();
        let bass = self.bass.get();
        let mid = self.mid.get();

        // Treble pot:
        // T --- Rtop --- OUT --- Rbottom --- C
        let r_treble_top = (r_treble_total * (1.0 - treble)).max(min_r);
        let r_treble_bottom = (r_treble_total * treble).max(min_r);

        stamp_resistor(
            &mut g,
            Some(Node::T as usize),
            Some(Node::Out as usize),
            r_treble_top,
        );

        stamp_resistor(
            &mut g,
            Some(Node::Out as usize),
            Some(Node::C as usize),
            r_treble_bottom,
        );

        // Bass pot:
        // C --- Rbass --- M
        //
        // У цій топології якщо напрям ручки буде інвертований,
        // просто заміни bass на (1.0 - bass).
        let r_bass = (r_bass_total * (1.0 - bass)).max(min_r);

        stamp_resistor(
            &mut g,
            Some(Node::C as usize),
            Some(Node::M as usize),
            r_bass,
        );

        // Mid pot:
        // M --- Rmid --- GND
        //
        // Більший mid = менше опору до землі = більше середини.
        let r_mid = (r_mid_total * (1.0 - mid)).max(min_r);

        stamp_resistor(&mut g, Some(Node::M as usize), GND, r_mid);

        // Load:
        // OUT -> GND
        stamp_resistor(&mut g, Some(Node::Out as usize), GND, r_load);

        // -------------------------
        // Capacitors
        // -------------------------

        self.c_treble.stamp(&mut g, &mut rhs);
        self.c_bass.stamp(&mut g, &mut rhs);
        self.c_mid.stamp(&mut g, &mut rhs);

        // -------------------------
        // Solve G * V = rhs
        // -------------------------

        let v = solve_6x6(g, rhs);

        // -------------------------
        // Update capacitor states
        // -------------------------

        self.c_treble.update(&v);
        self.c_bass.update(&v);
        self.c_mid.update(&v);

        v[Node::Out as usize]
    }
}

fn stamp_resistor(g_matrix: &mut [[f32; N]; N], n1: Option<usize>, n2: Option<usize>, r: f32) {
    let conductance = 1.0 / r;
    stamp_conductance(g_matrix, n1, n2, conductance);
}

fn stamp_conductance(
    g_matrix: &mut [[f32; N]; N],
    n1: Option<usize>,
    n2: Option<usize>,
    conductance: f32,
) {
    match (n1, n2) {
        (Some(a), Some(b)) => {
            g_matrix[a][a] += conductance;
            g_matrix[b][b] += conductance;
            g_matrix[a][b] -= conductance;
            g_matrix[b][a] -= conductance;
        }
        (Some(a), None) => {
            g_matrix[a][a] += conductance;
        }
        (None, Some(b)) => {
            g_matrix[b][b] += conductance;
        }
        (None, None) => {}
    }
}

fn solve_6x6(mut a: [[f32; N]; N], mut b: [f32; N]) -> [f32; N] {
    for i in 0..N {
        // Pivot
        let mut max_row = i;
        let mut max_val = a[i][i].abs();

        for row in (i + 1)..N {
            if a[row][i].abs() > max_val {
                max_val = a[row][i].abs();
                max_row = row;
            }
        }

        if max_row != i {
            a.swap(i, max_row);
            b.swap(i, max_row);
        }

        let pivot = a[i][i];

        if pivot.abs() < 1e-20 {
            continue;
        }

        // Normalize row
        for col in i..N {
            a[i][col] /= pivot;
        }

        b[i] /= pivot;

        // Eliminate
        for row in 0..N {
            if row == i {
                continue;
            }

            let factor = a[row][i];

            for col in i..N {
                a[row][col] -= factor * a[i][col];
            }

            b[row] -= factor * b[i];
        }
    }

    b
}
