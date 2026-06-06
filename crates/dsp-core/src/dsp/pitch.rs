pub fn detect_pitch(samples: &[f32], sample_rate: f32) -> Option<f32> {
    let half_len = samples.len() / 2;
    if half_len < 2 {
        return None;
    }

    let mut diff: Vec<f32> = Vec::with_capacity(half_len);

    // STEP 1: Squared Difference
    for lag in 0..half_len {
        let mut sum = 0.0;
        for i in 0..half_len {
            let curr_diff = (samples[i] - samples[i + lag]).powi(2);
            sum += curr_diff;
        }
        diff.push(sum);
    }

    // STEP 2: Cumulative Mean Normalized Difference
    let mut running_sum = 0.0;
    diff[0] = 1.0;

    for tau in 1..half_len {
        running_sum += diff[tau];
        if running_sum != 0.0 {
            diff[tau] /= running_sum / (tau as f32);
        } else {
            diff[tau] = 1.0;
        }
    }

    // STEP 3: Пошук першого мінімуму нижче порогу
    let threshold = 0.15; // Стандартний поріг для YIN (можна крутити 0.1 - 0.15)
    let mut candidate_index = None;

    // Шукаємо перший локальний мінімум, який менший за threshold
    for i in 1..(half_len - 1) {
        if diff[i] < threshold {
            // Перевіряємо, чи це локальний мінімум (менший за сусідів)
            if diff[i] < diff[i - 1] && diff[i] < diff[i + 1] {
                candidate_index = Some(i);
                break; // Знайшли перший найкращий період — виходимо!
            }
        }
    }

    // Якщо явного мінімуму нижче порогу немає, беремо просто абсолютний мінімум
    let candidate_index = candidate_index.unwrap_or_else(|| {
        let mut min_idx = 1;
        for i in 2..(half_len - 1) {
            if diff[i] < diff[min_idx] {
                min_idx = i;
            }
        }
        min_idx
    });

    // Захист від занадто малих лагів (щоб не ділити на 0 і не отримати артефакти)
    if candidate_index < 2 || candidate_index >= half_len - 1 {
        return None;
    }

    // STEP 4: Правильна параболічна інтерполяція (Standard parabolic fit)
    let alpha = diff[candidate_index - 1];
    let beta = diff[candidate_index];
    let gamma = diff[candidate_index + 1];

    let denominator = 2.0 * (alpha - 2.0 * beta + gamma);
    let delta = if denominator != 0.0 {
        (alpha - gamma) / denominator
    } else {
        0.0
    };

    // Додаємо дельту до ІНДЕКСУ (часу затримки), а не до значення функції
    let period = candidate_index as f32 + delta;

    if period > 0.0 {
        let freq = sample_rate / period;
        Some(freq)
    } else {
        None
    }
}

pub fn hz_to_note(freq: f32) -> Option<String> {
    if freq <= 0.0 {
        return None;
    }

    let midi_num = 69.0 + 12.0 * (freq / 440.0).log2();

    let midi_round = midi_num.round() as i32;

    if midi_round < 0 || midi_round > 127 {
        return None;
    }

    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    let note_index = (midi_round % 12) as usize;
    let octave = (midi_round / 12) - 1;

    Some(format!("{}{}", note_names[note_index], octave))
}
