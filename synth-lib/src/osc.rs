use super::{wave_table_osc::WavetableOscillator, Float, Note};
use crate::{lowpass::LowPassFilter, N_OVERTONES};
use std::f32::consts::PI;

// const N_OSCILATORS: usize = 10;

pub struct Oscillator {
    overtones: Vec<WavetableOscillator>,
    filter: LowPassFilter,
    note: Note,
}

impl Oscillator {
    pub fn new(table_size: u16, sample_rate: u32) -> Self {
        let two_pi = PI * 2.0;

        let wave_table: Vec<Float> = (0..table_size)
            .map(|n| (two_pi as Float * n as Float / table_size as Float).sin())
            .collect();
        let oscsilator = WavetableOscillator::new(sample_rate, wave_table.clone());
        let overtones = (0..N_OVERTONES).map(|_| oscsilator.clone()).collect();
        let filter = LowPassFilter::new();

        Self {
            overtones,
            filter,
            note: 0.0,
        }
    }

    pub fn get_sample(&mut self) -> Float {
        let sample: Float = self.overtones.iter_mut().map(|osc| osc.get_sample()).sum();

        self.filter
            .get_sample((sample / (N_OVERTONES as Float).sqrt()).tanh())
    }

    pub fn set_note(&mut self, note: Note) {
        self.note = note;

        self.overtones
            .iter_mut()
            .enumerate()
            .for_each(|(i, osc)| osc.set_frequency(note * (i as Float + 1.0)));
    }
}
