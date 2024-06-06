use super::echo::Echo;
use super::trem::Tremolo;
use super::wave_table_osc::WavetableOscillator;
use super::Float;
use super::{DISCOUNT, N_OSCILATORS};
use crate::envelope::Envelope;
use crate::envelope::{adbdr::ADBDREnvelope, adsr::ADSREnvelope, adsr_new::Filter};
use crate::notes::PITCH_BEND;
use log::error;
use std::f64::consts::PI;
use std::ops::Deref;

// const DISCOUNT: Float = 1.0 / (N_OSCILATORS as Float);
// const HALF_U16: Float = u16::MAX as Float * 0.5;
const VOLUME: Float = 1.0;

pub struct Synth {
    osc_s: Vec<WavetableOscillator>, // vectors iterate faster when using iter_mut apparently
    pub envelopes: [Filter; N_OSCILATORS],
    notes: [Float; N_OSCILATORS],
    // tremolo: Tremolo,
    // pub echo: Echo,
    pub volume: Float,
}

impl Synth {
    pub fn new(table_size: u16, sample_rate: u32) -> Self {
        let two_pi = PI * 2.0;

        let wave_table: Vec<Float> = (0..table_size)
            .map(|n| (two_pi as Float * n as Float / table_size as Float).sin())
            .collect();
        let oscsilator = WavetableOscillator::new(sample_rate, wave_table.clone());
        let osc_s = (0..N_OSCILATORS).map(|_| oscsilator.clone()).collect();
        // let osc_s = (1..=N_OSCILATORS)
        //     .map(|i| (oscsilator.clone(), 1.0 - (0.15 * i as Float)))
        //     .collect();
        let envelopes: [Filter; N_OSCILATORS] = [Filter::new(); N_OSCILATORS];
        // (0..N_OSCILATORS).for_each(|i| envelopes[i] = ADSREnvelope::new());
        let notes = [0.0; N_OSCILATORS];
        // let tremolo = Tremolo::new(oscsilator.clone());
        // let echo = Echo::new(sample_rate);

        Self {
            osc_s,
            envelopes,
            notes,
            // tremolo,
            // echo,
            volume: VOLUME,
        }
    }

    #[allow(dead_code)]
    pub fn set_frequency(&mut self, frequency: Float) {
        for (i, osc) in self.osc_s.iter_mut().enumerate() {
            osc.set_frequency(frequency * (i as Float + 1.0));
        }
    }

    /// used to start playing a note
    pub fn play(&mut self, note: Float) {
        if self.notes.contains(&note) {
            return;
        }
        // info!("playing note {note}");

        if let Some(i) = self.notes.iter().position(|freq| *freq == 0.0) {
            // if let Some(i) = self.envelopes.iter().position(|env| env.available()) {
            self.notes[i] = note;
            self.osc_s[i].set_frequency(note);
            self.envelopes[i].press();
        } else {
            error!("no oscilators available");
        }
    }

    /// used to start playing a note
    pub fn stop(&mut self, note: Float) {
        if let Some(i) = self.notes.iter().position(|freq| *freq == note) {
            self.notes[i] = 0.0;
            // self.osc_s[i].set_frequency(0.0);
            self.envelopes[i].release();
        }
    }

    // /// turns tremolo on or off
    // pub fn tremolo(&mut self, on: bool) {
    //     self.tremolo.set_status(on);
    // }
    //
    // pub fn tremolo_toggle(&mut self) {
    //     self.tremolo.toggle();
    // }
    //
    // /// turns echo on or off
    // pub fn echo(&mut self, on: bool) {
    //     self.echo.on(on);
    // }
    //
    // pub fn echo_toggle(&mut self) {
    //     self.echo.toggle();
    // }
    //
    // /// expects a number greater then zero. works best with numbers under 15
    // pub fn set_trem_freq(&mut self, frequency: Float) {
    //     self.tremolo.osc.set_frequency(frequency);
    // }
    //
    // /// expects a number between 0.0 and 1.0
    // pub fn set_trem_depth(&mut self, depth: Float) {
    //     self.tremolo.depth = depth;
    // }

    // fn convert(&mut self, sample: Float) -> (u8, u8) {
    fn convert(&mut self, sample: Float) -> [u8; 4] {
        debug_assert!(sample < 1.0);
        debug_assert!(sample > -1.0);
        // let trem_volume = if self.tremolo.on {
        //     self.tremolo.get_sample()
        // } else {
        //     1.0
        // };

        // let normal = (((sample + 1.0) * 0.5) * U16_MAX) as u16;
        // let normalized = ((if self.echo.on {
        //     self.echo.pop(sample)
        // } else {
        //     sample
        // } + 1.0)
        //     * 0.5)
        //     * HALF_U16;
        // let normalized =
        //     (sample + 1.0)
        //     * 0.5)
        //     * HALF_U16;
        //
        // // let converted = (normalized * ((self.volume + trem_volume) * 0.5)) as u16;
        // let converted = (normalized * self.volume * trem_volume) as u16;
        //
        // (
        //     (converted & 0b_0000_0000_1111_1111_u16) as u8,
        //     (converted >> 8) as u8,
        // )
        sample.to_le_bytes()
    }

    // pub fn get_sample(&mut self) -> (u8, u8) {
    pub fn get_sample(&mut self) -> [u8; 4] {
        let sample = self
            .osc_s
            .iter_mut()
            .zip(self.envelopes.iter_mut())
            // .enumerate()
            .map(|(osc, env)| {
                // if !(i == (N_OSCILATORS - 1) && self.tremolo.on) {
                // let s = osc.get_sample();
                // let e = env.envelope();
                // // println!("{s} {e}");
                osc.get_sample() * env.envelope()
                // } else {
                //     0.0
                // }
            })
            .sum::<Float>()
            * DISCOUNT;
        // let sample = self
        //     .osc_s
        //     .iter_mut()
        //     .map(|osc| osc.get_sample())
        //     .sum::<Float>()
        //     * DISCOUNT;
        self.convert(sample.tanh())
    }

    pub fn pitch_bend(&mut self, amount: Float) {
        for (i, osc) in self.osc_s.iter_mut().enumerate() {
            let note = self.notes[i];

            if note != 0.0 {
                let next_note = if amount < 0.0 {
                    note * PITCH_BEND.deref()
                } else {
                    note / PITCH_BEND.deref()
                };

                let distance = (note - next_note).abs() * amount;
                let new_note = note - distance;

                osc.set_frequency(new_note);
            }
        }
    }
}
