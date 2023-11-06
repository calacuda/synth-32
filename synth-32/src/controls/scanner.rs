use super::effect_conf::GenEffectConf;
use super::Octave;
use crate::notes::{NOTES, NOTE_NAMES};
use crate::Float;
use anyhow::Result;
use esp_idf_svc::hal::{
    adc::{self, AdcDriver, ADC1},
    gpio::*,
};
use synth::synth::Synth;
// use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{AnyIOPin, AnyOutputPin, Input, Output, PinDriver, Pull};
use log::*;
use std::sync::{Arc, Mutex};

enum Effect {
    Tremolo,
    Echo,
}

pub struct ADC {
    pub driver: AdcDriver<'static, ADC1>,
    pub vol: adc::AdcChannelDriver<'static, 3, Gpio4>,
    // pub trem_vol: adc::AdcChannelDriver<'static, 3, Gpio14>,
    // pub trem_speed: adc::AdcChannelDriver<'static, 3, Gpio15>,
    // pub echo_vol: adc::AdcChannelDriver<'static, 3, Gpio12>,
    // pub echo_speed: adc::AdcChannelDriver<'static, 3, Gpio13>,
}

pub struct Scanner {
    // pub pressed: Vec<KeyIndex>,
    pub octave: Octave,
    pub tick_i: u8,
    pub trem_conf: GenEffectConf,
    pub echo_conf: GenEffectConf,
    pub octave_up: bool,
    pub octave_down: bool,
    // pub columns: Vec<PinDriver<'static, AnyOutputPin, Output>>,
    pub columns: [PinDriver<'static, AnyOutputPin, Output>; 8],
    // pub rows: Vec<PinDrivepiano notes chartr<'static, AnyInputPin, Input>>,
    pub rows: [PinDriver<'static, AnyIOPin, Input>; 4],
    pub adc: ADC,
    pub synth: Arc<Mutex<Synth>>,
    pub vol: Float,
    // pub trem_vol: Float,
    // pub trem_speed: Float,
    // pub echo_vol: Float,
    // pub echo_speed: Float,
}

impl Scanner {
    pub fn step(&mut self) -> Result<()> {
        for col_i in 0..self.columns.len() {
            // 0..6
            self.columns[col_i].set_high()?;

            for row_i in 0..self.rows.len() {
                // 0..5
                if self.rows[row_i].is_high() {
                    // info!("key pressed at location: ({row_i}, {col_i}). [(row_i, col_i)]");
                    match (row_i, col_i) {
                        (2, 1) => self.octave_down = true,       // self.octave_down(),
                        (4, 0) => self.octave_up = true,         // self.octave_up(),
                        (3, 0) => self.echo_conf.pressed = true, // self.toggle(Effect::Echo),
                        (2, 0) => self.trem_conf.pressed = true, // self.toggle(Effect::Tremolo),
                        (r, c) => {
                            self.play_note(r, c);
                            // println!("{}", self.get_i(r, c));
                        }
                    }
                } else {
                    match (row_i, col_i) {
                        (2, 1) => self.octave_down(),
                        (4, 0) => self.octave_up(),
                        (3, 0) => self.toggle(Effect::Echo),
                        (2, 0) => self.toggle(Effect::Tremolo),
                        (r, c) => self.stop_note(r, c),
                    }
                }
            }

            self.columns[col_i].set_low()?;
            // FreeRtos::delay_us(100);
        }

        // TODO: average three readings from each knob and change effect settings accordingly
        self.vol += ((self.adc.driver.read(&mut self.adc.vol).unwrap() - 128) as Float) / 3024.0;
        // self.trem_vol +=
        //     (self.adc.driver.read(&mut self.adc.trem_vol).unwrap() - 128) as Float / 3024.0;
        // self.echo_vol +=
        //     (self.adc.driver.read(&mut self.adc.echo_vol).unwrap() - 128) as Float / 3024.0;
        // self.trem_speed +=
        //     ((self.adc.driver.read(&mut self.adc.trem_speed).unwrap() - 128) as Float / 3024.0)
        //         * 15.0;
        // self.echo_speed +=
        //     ((self.adc.driver.read(&mut self.adc.echo_speed).unwrap() - 128) as Float / 3024.0)
        //         * 4.0
        //         + 1.0;

        if self.tick_i == 0 {
            let n_ticks = 10.0;
            self.synth.lock().unwrap().volume = self.vol / n_ticks;
            self.vol = 0.0;
        }

        self.tick_i += 1;
        self.tick_i %= 10;

        Ok(())
    }

    fn octave_down(&mut self) {
        if self.octave > 0 && self.octave_down {
            self.octave_down = false;
            self.octave -= 1;
        }
    }

    fn octave_up(&mut self) {
        if self.octave < 8 && self.octave_up {
            self.octave_up = false;
            self.octave += 1;
        }
    }

    fn toggle(&mut self, effect: Effect) {
        match effect {
            Effect::Tremolo => {
                if self.trem_conf.pressed {
                    self.trem_conf.pressed = false;
                    self.synth.lock().unwrap().tremolo_toggle();
                }
            }
            Effect::Echo => {
                if self.echo_conf.pressed {
                    self.echo_conf.pressed = false;
                    self.synth.lock().unwrap().echo_toggle();
                }
            }
        }
    }

    fn get_i(&mut self, r: usize, c: usize) -> usize {
        ((self.columns.len() - 1) * c + r) + (12 * self.octave as usize)
    }

    fn get_freq(&mut self, r: usize, c: usize) -> Float {
        // info!("note: {}", NOTE_NAMES[i]);
        let i = self.get_i(r, c);
        NOTES.get(NOTE_NAMES[i]).unwrap().clone()
    }

    fn play_note(&mut self, r: usize, c: usize) {
        let note = self.get_freq(r, c);
        self.synth.lock().unwrap().play(note);
    }

    fn stop_note(&mut self, r: usize, c: usize) {
        let note = self.get_freq(r, c);
        self.synth.lock().unwrap().stop(note);
    }

    pub fn set_pull(&mut self) -> Result<()> {
        for pin in self.rows.iter_mut() {
            pin.set_pull(Pull::Down)?;
        }

        Ok(())
    }
}
