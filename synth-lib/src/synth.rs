use super::Float;
use super::N_OSCILATORS;
use crate::notes::PITCH_BEND;
use crate::osc::Oscillator;
use crate::DISCOUNT;
use log::*;
use std::ops::Deref;

const VOLUME: Float = 0.9;
const HALF_U32: Float = u32::MAX as Float;

pub struct Synth {
    // osc_s: Vec<WavetableOscillator>, // vectors iterate faster when using iter_mut apparently
    pub osc_s: Vec<Oscillator>, // vectors iterate faster when using iter_mut apparently
    // pub envelopes: [Filter; N_OSCILATORS],
    notes: [Float; N_OSCILATORS],
    // pub filter: LowPassFilter,
    // tremolo: Tremolo,
    // pub echo: Echo,
    pub volume: Float,
}

impl Synth {
    pub fn new(table_size: u16, sample_rate: u32) -> Self {
        // let two_pi = PI * 2.0;

        // let wave_table: Vec<Float> = (0..table_size)
        //     .map(|n| (two_pi as Float * n as Float / table_size as Float).sin())
        //     .collect();
        let oscsilator = Oscillator::new(table_size, sample_rate);
        let osc_s = (0..N_OSCILATORS).map(|_| oscsilator.clone()).collect();
        // let osc_s = (1..=N_OSCILATORS)
        //     .map(|i| (oscsilator.clone(), 1.0 - (0.15 * i as Float)))
        //     .collect();
        // let envelopes: [Filter; N_OSCILATORS] = [Filter::new(); N_OSCILATORS];
        // (0..N_OSCILATORS).for_each(|i| envelopes[i] = ADSREnvelope::new());
        let notes = [0.0; N_OSCILATORS];
        // let tremolo = Tremolo::new(oscsilator.clone());
        // let echo = Echo::new(sample_rate);
        // let filter = LowPassFilter::new();

        Self {
            osc_s,
            // envelopes,
            notes,
            // tremolo,
            // echo,
            // filter,
            volume: VOLUME,
        }
    }

    #[allow(dead_code)]
    pub fn set_frequency(&mut self, frequency: Float) {
        for (i, osc) in self.osc_s.iter_mut().enumerate() {
            osc.set_note(frequency * (i as Float + 1.0));
        }
    }

    /// used to start playing a note
    pub fn play(&mut self, note: Float) {
        if self.notes.contains(&note) {
            return;
        }
        // info!("playing note {note}");

        // if let Some(i) = self.notes.iter().position(|freq| *freq == 0.0) {
        if let Some(i) = self.osc_s.iter().position(|osc| osc.env.available()) {
            // warn!("using osc {i}");
            self.notes[i] = note;
            self.osc_s[i].set_note(note);
            self.osc_s[i].env.press();
        } else {
            error!("no oscilators available");
        }
    }

    /// used to start playing a note
    pub fn stop(&mut self, note: Float) {
        if let Some(i) = self.notes.iter().position(|freq| *freq == note) {
            self.notes[i] = 0.0;
            // self.osc_s[i].set_note(0.0);
            self.osc_s[i].env.release();
        }
    }

    pub fn set_cutoff(&mut self, cutoff: Float) {
        self.osc_s.iter_mut().for_each(|osc| osc.set_cutoff(cutoff));
    }

    pub fn set_resonance(&mut self, cutoff: Float) {
        self.osc_s
            .iter_mut()
            .for_each(|osc| osc.set_resonance(cutoff));
    }

    pub fn set_attack(&mut self, attack: Float) {
        self.osc_s.iter_mut().for_each(|osc| {
            osc.env.set_attack(attack);
        })
    }

    pub fn set_decay(&mut self, decay: Float) {
        self.osc_s.iter_mut().for_each(|osc| {
            osc.env.set_decay(decay);
        })
    }

    pub fn set_sustain(&mut self, sustain: Float) {
        self.osc_s.iter_mut().for_each(|osc| {
            osc.env.set_sustain(sustain);
        })
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

        let normalized = (sample + 1.0) * HALF_U32 * self.volume;
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

        // let converted = (normalized * ((self.volume + trem_volume) * 0.5)) as u16;
        let converted = normalized as u32;

        // (
        //     (converted & 0b_0000_0000_1111_1111_u16) as u8,
        //     (converted >> 8) as u8,
        // )

        // let normalized = (sample + 1.0) * 0.5 * HALF_U32;

        // let converted = (normalized * self.volume) as u32;

        converted.to_le_bytes()
    }

    // pub fn get_sample(&mut self) -> (u8, u8) {
    pub fn get_sample(&mut self) -> [u8; 4] {
        let sample = self
            .osc_s
            .iter_mut()
            // .zip(self.envelopes.iter_mut())
            // .enumerate()
            .map(|osc| {
                // if !(i == (N_OSCILATORS - 1) && self.tremolo.on) {
                // let s = osc.get_sample();
                // let e = env.envelope();
                // // println!("{s} {e}");
                // let e  = env.get_envelope();
                // info!("envlepesqrt of 7: {i} has value: {e}");
                // info!("envelope value => {e}");
                // if e > 0.0 && e < 0.01 {
                //     info!("envelope value => {e}");
                // } else if e > 0.98 {
                //     info!("envelope value => {e}");
                // }
                // println!("{e}");
                // let s = osc.get_sample();

                // let sample = s * e;

                // if sample != s {
                //     warn!("{s} * {e} = {sample}");
                // }
                // println!("{sample}");

                // sample
                let (osc_sample, env_sample) = osc.get_sample();
                // env.get_envelope()
                // } else {
                //     0.0
                // }

                osc_sample * env_sample
            })
            .sum::<Float>()
            * DISCOUNT
            // * self.volume;
        ;
        // let sample = self
        //     .osc_s
        //     .iter_mut()
        //     .map(|osc| osc.get_sample())
        //     .sum::<Float>()
        //     * DISCOUNT;
        // let sample = self.filter.get_sample(sample.tanh());
        // let osc_sample = self.osc_s[0].get_sample();
        // let env_sample = self.envelopes[0].get_envelope();
        // info!("{env_sample}");
        // info!("{}", 20.0 * env_sample.log10());
        // if osc_sample > 1.0 {
        //     info!("{osc_sample}");
        // }
        // let mut sample = osc_sample;
        // println!("{osc_sample} * {env_sample} = {sample}");
        // if sample = osc_sample && sample != 0.0 {
        //     error!("{sample}");
        // }
        // let sample = ((sample_f + 1.0) * 0.5) * HALF_U32; // * env_sample * self.volume;
        // println!("{sample}");
        // if sample == -0.0 {
        //     sample = 0.0;
        // }
        self.convert(sample)
        // let as_bytes = sample.to_le_bytes();
        //
        // println!("sample: {:?}", sample);
        // println!("i32: {:?}", i32::from_le_bytes(as_bytes));
        //
        // as_bytes
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

                osc.set_note(new_note);
            }
        }
    }
}
