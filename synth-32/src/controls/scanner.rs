use super::effect_conf::GenEffectConf;
use super::Octave;
use crate::Float;
use anyhow::Result;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::{AnyIOPin, AnyOutputPin, Input, Output, PinDriver, Pull};
use esp_idf_svc::hal::{
    adc::{self, AdcDriver, ADC1, ADC2},
    gpio::*,
};
use log::*;
// use std::collections::HashMap;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use synth::envelope::Envelope;
use synth::notes::{NOTES, NOTE_NAMES};
use synth::synth::Synth;

const N_TICKS: u8 = 40;

pub struct ADC {
    pub driver1: AdcDriver<'static, ADC1>,
    pub driver2: AdcDriver<'static, ADC2>,
    pub pitch: adc::AdcChannelDriver<'static, 3, Gpio4>,
    pub vol: adc::AdcChannelDriver<'static, 3, Gpio5>,
    pub trem_vol: adc::AdcChannelDriver<'static, 3, Gpio16>,
    pub trem_speed: adc::AdcChannelDriver<'static, 3, Gpio15>,
    pub echo_vol: adc::AdcChannelDriver<'static, 3, Gpio18>,
    pub echo_speed: adc::AdcChannelDriver<'static, 3, Gpio17>,
    pub attack: adc::AdcChannelDriver<'static, 3, Gpio7>,
    pub decay: adc::AdcChannelDriver<'static, 3, Gpio6>,
}

pub struct Buttons {
    pub octave_up: PinDriver<'static, AnyInputPin, Input>,
    pub octave_down: PinDriver<'static, AnyInputPin, Input>,
    pub tremolo: PinDriver<'static, AnyInputPin, Input>,
    pub echo: PinDriver<'static, AnyInputPin, Input>,
}

pub struct Scanner {
    pub pressed: BTreeMap<(usize, usize), usize>, //  [[Option<usize>; 8]; 4],
    pub octave: Octave,
    pub tick_i: u8,
    pub trem_conf: GenEffectConf,
    pub echo_conf: GenEffectConf,
    pub octave_up: bool,
    pub octave_down: bool,
    pub columns: [PinDriver<'static, AnyOutputPin, Output>; 8],
    pub rows: [PinDriver<'static, AnyIOPin, Input>; 4],
    pub adc: ADC,
    pub synth: Arc<Mutex<Synth>>,
    pub vol: Float,
    pub attack: Float,
    pub decay: Float,
    pub pitch: i32,
    pub buttons: Buttons,
    // pub trem_vol: Float,
    // pub trem_speed: Float,
    // pub echo_vol: Float,
    // pub echo_speed: Float,
}

impl Scanner {
    /// used to step through the state of the keyboard and change audio synthesis settings
    /// acourdingly.
    pub fn step(&mut self) -> Result<()> {
        self.octave_up();
        self.octave_down();
        // info!("finding pressed keys");
        // figure out what keys were pressed.
        for col_i in 0..self.columns.len() {
            self.columns[col_i].set_high()?;

            for row_i in 0..self.rows.len() {
                // println!("({row_i}/{col_i}) => {}", self.rows[row_i].is_high());
                // play/stop note
                if self.rows[row_i].is_high() && !self.pressed.contains_key(&(row_i, col_i)) {
                    // info!("key pressed at location: ({row_i}, {col_i}). [(row_i, col_i)]");
                    self.play_note(row_i, col_i);
                } else if self.rows[row_i].is_low() && self.pressed.contains_key(&(row_i, col_i)) {
                    self.stop_note(row_i, col_i);
                }
            }

            self.columns[col_i].set_low()?;
        }
        // info!("found pressed keys");

        // read the knob position and average it.
        self.pitch += self.adc.driver1.read(&mut self.adc.pitch)? as i32;
        let vol_reading = self.adc.driver1.read(&mut self.adc.vol)?;
        self.vol = (self.vol + self.normalize_reading(vol_reading)) * 0.5;
        let attack_reading = self.adc.driver1.read(&mut self.adc.attack)?;
        self.attack = (self.attack + self.normalize_reading(attack_reading)) * 0.5;
        let decay_reading = self.adc.driver1.read(&mut self.adc.decay)?;
        self.decay = (self.decay + self.normalize_reading(decay_reading)) * 0.5;

        // let trem_vol_reading = self.adc.driver2.read(&mut self.adc.trem_vol)?;
        // self.trem_conf.volume =
        //     (self.trem_conf.volume + self.normalize_big_reading(trem_vol_reading)) * 0.5;
        // let trem_speed_reading = self.adc.driver2.read(&mut self.adc.trem_speed)?;
        // self.trem_conf.speed =
        //     (self.trem_conf.speed + self.normalize_big_reading(trem_speed_reading)) * 0.5;
        // let echo_vol_reading = self.adc.driver2.read(&mut self.adc.echo_vol)?;
        // self.echo_conf.volume =
        //     (self.echo_conf.volume + self.normalize_big_reading(echo_vol_reading)) * 0.5;
        // let echo_speed_reading = self.adc.driver2.read(&mut self.adc.echo_speed)?;
        // self.echo_conf.speed =
        //     (self.echo_conf.speed + self.normalize_big_reading(echo_speed_reading)) * 0.5;

        // info!("knobs read");

        if self.tick_i == (N_TICKS - 1) {
            let bend = ((self.pitch / N_TICKS as i32) - 962) - 670;

            if 2 < bend || bend < -2 {
                self.synth
                    .lock()
                    .unwrap()
                    .pitch_bend(bend as Float / 1350.0);
            }

            self.synth.lock().unwrap().volume = self.vol;
            // self.synth
            //     .lock()
            //     .unwrap()
            //     .set_trem_freq(self.trem_conf.speed * 11.0);
            // self.synth
            //     .lock()
            //     .unwrap()
            //     .set_trem_depth(self.trem_conf.volume);
            // self.synth
            //     .lock()
            //     .unwrap()
            //     .echo
            //     .set_speed((self.echo_conf.speed * 100.0).round() / 100.0);
            // self.synth
            //     .lock()
            //     .unwrap()
            //     .echo
            //     .set_volume(self.echo_conf.volume);
            self.set_attack();
            self.set_decay();

            // set effects
            // if self.buttons.tremolo.is_low() {
            //     self.synth.lock().unwrap().tremolo(true);
            // } else {
            //     self.synth.lock().unwrap().tremolo(false);
            // }

            // if self.buttons.echo.is_low() && !self.echo_conf.pressed {
            //     // info!("echo on");
            //     // info!("{}", self.synth.lock().unwrap().echo.);
            //     self.synth.lock().unwrap().echo(true);
            //     self.echo_conf.pressed = true;
            // } else if self.buttons.echo.is_high() && self.echo_conf.pressed {
            //     self.synth.lock().unwrap().echo(false);
            //     self.echo_conf.pressed = false;
            // }

            // reset pitch
            self.pitch = 0;
            self.vol = 0.0;
            // self.trem_conf.volume = 0.0;
            // self.trem_conf.speed = 0.0;
            // self.echo_conf.volume = 0.0;
            // self.echo_conf.speed = 0.0;
        }

        // info!("incrementing tick_i");

        self.tick_i += 1;
        self.tick_i %= N_TICKS;
        // FreeRtos::delay_us(10);

        Ok(())
    }

    /// used to shift the keyboard down an octave on the release of the button
    fn octave_down(&mut self) {
        if self.buttons.octave_down.is_low() && !self.octave_down {
            // info!("octave down pressed");
            self.octave_down = true;
        } else if self.buttons.octave_down.is_high() && self.octave_down && self.octave > 0 {
            self.octave_down = false;
            self.octave -= 1;
            // info!("octave_down");
        }
    }

    /// used to shift the keyboard up an octave on the release of the button
    fn octave_up(&mut self) {
        if self.buttons.octave_up.is_low() && !self.octave_up {
            // info!("octave up pressed");
            self.octave_up = true;
        } else if self.buttons.octave_up.is_high() && self.octave_up && self.octave < (8 - 1) {
            self.octave_up = false;
            self.octave += 1;
            // info!("octave_up");
        }
    }

    /// function for use internal to this strunct. it translates row and column index to an index
    /// into a list of note names
    fn get_i(&mut self, r: usize, c: usize) -> usize {
        ((self.columns.len() * r) + c) + (12 * self.octave as usize)
    }

    fn get_freq(&mut self, r: usize, c: usize) -> Option<(usize, Option<&Float>)> {
        let i = self.get_i(r, c);
        if i >= NOTE_NAMES.len() {
            return None;
        }
        Some((i, NOTES.get(NOTE_NAMES[i])))
    }

    fn play_note(&mut self, r: usize, c: usize) {
        if self.pressed.get(&(r, c)).is_some() {
            return;
        }

        if let Some((i, Some(note))) = self.get_freq(r, c) {
            info!("{note}");
            let note = note.clone();
            self.synth.lock().unwrap().play(note);
            self.pressed.insert((r, c), i);
        } else {
            info!("not note for ({r}/{c})");
        }
    }

    fn stop_note(&mut self, r: usize, c: usize) {
        if let Some(i) = self.pressed.get(&(r, c)) {
            let note = NOTES.get(NOTE_NAMES[*i]).unwrap().clone();
            self.synth.lock().unwrap().stop(note);
            self.pressed.remove(&(r, c));
        }
    }

    fn set_pull(&mut self) -> Result<()> {
        // self.rows.iter_mut().for_each(|pin| {
        //     if let Err(e) = pin.set_pull(Pull::Down) {
        //         error!("settings row pin to pull down failed with error {e}");
        //     }
        // });
        for pin in self.rows.iter_mut() {
            if let Err(e) = pin.set_pull(Pull::Down) {
                error!("settings row pin to pull down failed with error {e}");
            }
        }

        // self.buttons.octave_up.set_pull(Pull::Down)?;

        info!("pin pull set successfully");

        Ok(())
    }

    fn set_attack(&mut self) {
        self.synth
            .lock()
            .unwrap()
            .osc_s
            .iter_mut()
            .for_each(|osc| {
                osc.env.set_attack(self.attack);
            });
        self.attack = 0.0;
    }

    fn set_decay(&mut self) {
        self.synth
            .lock()
            .unwrap()
            .osc_s
            .iter_mut()
            .for_each(|osc| {
                osc.env.set_decay(self.decay);
            });
        self.decay = 0.0;
    }

    fn normalize_reading(&mut self, reading: u16) -> Float {
        reading as Float / 3134.0
    }

    fn normalize_big_reading(&mut self, reading: u16) -> Float {
        reading as Float / 4972.0
    }

    // fn normalize_small_reading(&mut self, reading: u16) -> Float {
    //     reading as Float / 2320.0
    // }

    pub fn init(&mut self) -> Result<()> {
        self.set_pull()?;

        // set initial pitch
        info!("setting modwheel base position");
        self.pitch = self.adc.driver1.read(&mut self.adc.pitch).unwrap() as i32;

        // set initial volume
        info!("setting volume");
        let vol = self.adc.driver1.read(&mut self.adc.vol)?;
        self.vol = self.normalize_reading(vol);
        self.synth.lock().unwrap().volume = self.vol;

        // TODO: set initial envelope filter parameters (hardcoded values for testing).
        
        info!("init done");
        Ok(())
    }
}
