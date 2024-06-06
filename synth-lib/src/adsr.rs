use super::Envelope;
use crate::{Float, SAMPLE_RATE};
use anyhow::{bail, Result};

// use tracing::*;

pub const N_INPUTS: u8 = 5;
pub const N_OUTPUTS: u8 = 1;

pub const ATTACK_IN: u8 = 3; // sets attack speed in seconds
pub const DECAY_IN: u8 = 4; // sets decay 1 speed in seconds
pub const DECAY_THRESHOLD: u8 = 5; // sets the threshold between decay 1 sustain in amplitude

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Phase {
    Neutural,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Debug, Clone)]
pub struct Filter {
    phase: Phase,
    env: Float,
    pub decay_speed: Float,
    pub attack_speed: Float,
    pub threshold: Float,
    pub decay: Float,
    pub attack: Float,
    pub release: Float,
    sample_rate: Float,
    pub pressed: bool,
    release_threshold: Float,
}

impl Filter {
    pub fn new() -> Self {
        let sample_rate = SAMPLE_RATE as Float;
        let attack_speed = 0.01;
        let attack = 1.0 / (sample_rate * attack_speed);
        let decay_speed = 0.1;
        let decay = -1.0 / (sample_rate * decay_speed);

        Self {
            // pressed: false,
            phase: Phase::Neutural,
            // i: 0,
            env: 0.0,
            decay,
            attack,
            threshold: 0.9,
            sample_rate,
            attack_speed,
            decay_speed,
            release: -0.9 / (sample_rate * 0.1),
            pressed: false,
            release_threshold: 0.05,
        }
    }

    fn set_attack(&mut self, atk_speed: Float) {
        if atk_speed != self.attack_speed {
            self.attack_speed = atk_speed;
            self.attack = 1.0 / (self.sample_rate * atk_speed);
        }
    }

    fn set_decay(&mut self, decay_speed: Float) {
        if decay_speed != self.decay_speed {
            self.decay_speed = decay_speed;
            self.decay = -1.0 / (self.sample_rate * decay_speed);
        }
    }

    fn set_threshold(&mut self, threshold: Float) {
        self.threshold = threshold;
        self.release = -threshold / (self.sample_rate * 0.01);
    }

    fn internal_update_phase(&mut self) {
        if self.phase == Phase::Attack && self.env >= 1.0 {
            self.phase = Phase::Decay;
            self.env = 1.0;
            // info!("changing phase to => {:?}", self.phase);
        } else if self.phase == Phase::Decay && self.env <= self.threshold {
            self.phase = Phase::Sustain;
            // info!("changing phase to => {:?}", self.phase);
            // } else if self.phase == Phase::Sustain && self.env <= self.release_threshold {
            //     self.phase = Phase::Release;
            // info!("changing phase to => {:?}", self.phase);
        } else if self.phase == Phase::Release && self.env <= 0.0 {
            self.phase = Phase::Neutural;
            self.env = 0.0;
            // info!("changing phase to => {:?}", self.phase);
        }
    }
}

impl Envelope for Filter {
    fn get_env(&mut self) -> Float {
        self.env
    }

    fn set_env(&mut self, env: Float) {
        self.env = env;
    }

    fn get_step(&mut self) -> Float {
        match self.phase {
            Phase::Attack => self.attack,
            Phase::Decay => self.decay,
            Phase::Sustain => 0.0,
            Phase::Release => self.release,
            Phase::Neutural => 0.0,
        }
    }

    fn update_phase(&mut self) {
        self.internal_update_phase()
    }

    fn open_filter(&mut self, samples: Vec<Float>) -> bool {
        let sample: Float = samples.iter().sum::<Float>().tanh();

        if self.pressed && sample <= 0.75 {
            // info!("release");
            self.phase = Phase::Release;
            self.pressed = false;
        } else if !self.pressed
            && (self.phase == Phase::Neutural || self.phase == Phase::Release)
            && sample >= 0.75
        {
            // info!("pressed");
            self.phase = Phase::Attack;
            self.pressed = true;
        }

        self.pressed
    }

    fn take_input(&mut self, input: u8, samples: Vec<Float>) -> Result<()> {
        let sample: Float = samples.iter().sum::<Float>();

        match input {
            // attack in
            0 => self.set_attack(sample),
            // decay speed in
            1 => self.set_decay(sample),
            // decay_threshold in
            2 => self.set_threshold(sample),
            n => bail!("{n} is not a valid input for the ADSR filter."),
        }

        Ok(())
    }

    fn pressed(&mut self) -> bool {
        self.phase != Phase::Neutural
        // self.pressed
    }
}
