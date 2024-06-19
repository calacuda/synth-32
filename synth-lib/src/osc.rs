use super::{wave_table_osc::WavetableOscillator, Float, Note};
use crate::{envelope::adsr_new::Filter, lowpass::LowPassFilter, N_OVERTONES};
use std::f32::consts::PI;

// const N_OSCILATORS: usize = 10;

#[derive(Clone)]
pub struct Oscillator {
    oscillator: WavetableOscillator,
    filter: LowPassFilter,
    pub env: Filter,
    note: Note,
}

impl Oscillator {
    pub fn new(table_size: u16, sample_rate: u32) -> Self {
        let two_pi = PI * 2.0;

        // let wave_table: Vec<Float> = (0..table_size)
        //     .map(|n| (two_pi as Float * n as Float / table_size as Float).sin())
        //     .collect();
        let wave_table: Vec<Float> = (0..table_size)
            .map(|t| {
                (1..N_OVERTONES)
                    .map(|overtone| {
                        (two_pi as Float * t as Float * overtone as Float / table_size as Float)
                            .sin()
                            / (N_OVERTONES as Float).sqrt()
                    })
                    .sum()
            })
            .collect();
        let oscillator = WavetableOscillator::new(sample_rate, wave_table.clone());
        // let overtones = (0..N_OVERTONES).map(|_| oscsilator.clone()).collect();
        let filter = LowPassFilter::new();
        let env = Filter::new();

        Self {
            oscillator,
            filter,
            env,
            note: 0.0,
        }
    }

    pub fn get_sample(&mut self) -> (Float, Float) {
        let sample: Float = self.oscillator.get_sample();
        let _ = self.env.get_envelope();

        (
            self.filter
                .get_sample(sample / (N_OVERTONES as Float).sqrt()),
            self.env.get_env(),
        ) // sample / (N_OVERTONES as Float)
    }

    pub fn set_note(&mut self, note: Note) {
        self.note = note;
        self.oscillator.set_frequency(note);

        // self.overtones
        //     .iter_mut()
        //     .enumerate()
        //     .for_each(|(i, osc)| osc.set_frequency(note * (i as Float + 1.0)));
    }

    pub fn set_cutoff(&mut self, cutoff: Float) {
        self.filter.set_cutoff(cutoff)
    }

    pub fn set_resonance(&mut self, resonance: Float) {
        self.filter.set_resonance(resonance)
    }
}
