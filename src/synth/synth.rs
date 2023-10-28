use super::trem::Tremolo;
use super::wave_table_osc::WavetableOscillator;
use super::Float;
use super::N_OSCILATORS;
// use log::*;
use std::f64::consts::PI;

const DISCOUNT: Float = 1.0 / N_OSCILATORS as Float;
const HALF_U16: Float = (u16::MAX / 2) as Float;
const VOLUME: Float = 0.25;

pub struct Synth {
    osc_s: Vec<WavetableOscillator>, // vectors iterate faster when using iter_mut apparently
    notes: [Float; N_OSCILATORS - 1],
    tremolo: Tremolo,
    volume: Float,
}

impl Synth {
    pub fn new(table_size: u16, sample_rate: u32) -> Self {
        let two_pi = PI * 2.0;

        let wave_table: Vec<Float> = (0..table_size)
            .map(|n| (two_pi as Float * n as Float / table_size as Float).sin())
            .collect();
        let oscsilator = WavetableOscillator::new(sample_rate, wave_table.clone());
        let osc_s = (1..N_OSCILATORS).map(|_| oscsilator.clone()).collect();
        // let osc_s = (1..=N_OSCILATORS)
        //     .map(|i| (oscsilator.clone(), 1.0 - (0.15 * i as Float)))
        //     .collect();
        let notes = [0.0; N_OSCILATORS - 1];
        let tremolo = Tremolo::new(oscsilator.clone());

        Self {
            osc_s,
            notes,
            tremolo,
            volume: VOLUME,
        }
    }

    pub fn set_frequency(&mut self, frequency: Float) {
        for (i, osc) in self.osc_s.iter_mut().enumerate() {
            osc.set_frequency(frequency * (i as Float + 1.0));
        }
    }

    /// used to start playing a note
    pub fn play(&mut self, note: Float) {
        if let Some(i) = self.notes.iter().position(|freq| *freq == 0.0) {
            self.notes[i] = note;
            self.osc_s[i].set_frequency(note);
        }
    }

    /// used to start playing a note
    pub fn stop(&mut self, note: Float) {
        if let Some(i) = self.notes.iter().position(|freq| *freq == note) {
            self.notes[i] = 0.0;
            self.osc_s[i].set_frequency(0.0);
        }
    }

    pub fn tremolo(&mut self, on: bool) {
        self.tremolo.set_status(on);
    }

    pub fn set_trem_freq(&mut self, frequency: Float) {
        self.tremolo.osc.set_frequency(frequency);
    }

    pub fn set_trem_depth(&mut self, depth: Float) {
        self.tremolo.depth = depth;
    }

    fn convert(&mut self, sample: Float) -> (u8, u8) {
        // (((sample + 1.0) / 2.0) * 255.0) as u8 // let sample = frame.channels()[0].to_f32();
        // (((sample * 0.5) + 0.5) * 255.0) as u8 // let sample = frame.channels()[0].to_f32();
        debug_assert!(sample < 1.0);
        debug_assert!(sample > -1.0);
        let volume = if self.tremolo.on {
            self.tremolo.get_sample()
        } else {
            1.0
        };

        // let normal = (((sample + 1.0) * 0.5) * U16_MAX) as u16;
        let normal = ((((sample + 1.0) * 0.5) * HALF_U16) * self.volume * volume) as u16;

        (
            (normal & 0b_0000_0000_1111_1111_u16) as u8,
            (normal >> 8) as u8,
        )
    }

    pub fn get_sample(&mut self) -> (u8, u8) {
        let sample = self
            .osc_s
            .iter_mut()
            .map(|osc| osc.get_sample())
            .sum::<Float>()
            * DISCOUNT;
        self.convert(sample)
    }
}
