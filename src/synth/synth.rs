use super::wave_table_osc::WavetableOscillator;
use super::Float;
use super::N_OSCILATORS;
// use log::*;
use std::f64::consts::PI;

const DISCOUNT: Float = 1.0 / N_OSCILATORS as Float;

pub struct Synth {
    osc_s: Vec<(WavetableOscillator, Float)>, // vectors iterate faster when using iter_mut apparently
}

impl Synth {
    pub fn new(table_size: u16, sample_rate: u32) -> Self {
        let two_pi = PI * 2.0;

        let wave_table: Vec<Float> = (0..table_size)
            .map(|n| (two_pi as Float * n as Float / table_size as Float).sin())
            .collect();
        let oscsilator = WavetableOscillator::new(sample_rate, wave_table.clone());
        let osc_s = (1..=N_OSCILATORS)
            .map(|i| (oscsilator.clone(), 1.0 - (0.15 * i as Float)))
            .collect();

        Self { osc_s }
    }

    pub fn set_frequency(&mut self, frequency: Float) {
        for (i, (osc, _)) in self.osc_s.iter_mut().enumerate() {
            // info!("settign frequency => {}", frequency * (i as Float + 1.0));
            osc.set_frequency(frequency * (i as Float + 1.0));
        }
    }

    // used when handling multiple notes
    // pub fn play(&mut self, note: Float) {
    //     if let Some(i) = self.notes.iter().position(|freq| *freq == 0.0) {
    //         self.notes[i] = note;
    //         self.osc_s[i].set_frequency(note);
    //         self.n_notes += 1.0;
    //         self.discount = 1.0 / (self.n_notes + 1.0);
    //     }
    // }
    //
    // pub fn stop(&mut self, note: Float) {
    //     if let Some(i) = self.notes.iter().position(|freq| *freq == note) {
    //         self.notes[i] = 0.0;
    //         self.osc_s[i].set_frequency(0.0);
    //         self.n_notes -= 1.0;
    //         self.discount = 1.0 / (self.n_notes + 0.5);
    //     }
    // }

    pub fn get_sample(&mut self) -> Float {
        self.osc_s
            .iter_mut()
            .map(|(osc, v)| osc.get_sample() * *v)
            .sum::<Float>()
            * DISCOUNT
    }
}
