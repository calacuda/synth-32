use log::info;

use super::Envelope;
use crate::Float;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Phase {
    Neutural,
    Attack,
    Decay1,
    Decay2,
    Release,
}

#[derive(Debug, Clone, Copy)]
pub struct ADBDREnvelope {
    pub pressed: bool,
    phase: Phase,
    i: usize,
    env: Float,
    decay: Float,
    attack: Float,
    sample_rate: Float,
}

impl ADBDREnvelope {
    pub fn new(sample_rate: Float) -> Self {
        Self {
            pressed: false,
            phase: Phase::Neutural,
            i: 0,
            env: 0.0,
            decay: 0.0,
            attack: 0.0,
            sample_rate,
        }
    }

    fn update_phase(&mut self) {
        if self.phase == Phase::Attack && self.env >= 1.0 {
            // info!("1");
            self.phase = Phase::Decay1;
            self.env = 1.0;
            self.i = 0;
        } else if self.phase == Phase::Decay1 && self.env <= self.decay {
            // info!("2");
            self.phase = Phase::Decay2;
            self.i = 0;
        } else if self.phase == Phase::Decay2 && self.env <= 0.1 {
            // info!("3");
            self.phase = Phase::Release;
            self.i = 0;
        } else if self.phase == Phase::Release && self.env <= 0.0 {
            self.phase = Phase::Neutural;
        }
    }
}

impl Envelope for ADBDREnvelope {
    fn envelope(&mut self) -> Float {
        // println!("{:?} => i: {}, env: {}", self.phase, self.i, self.env);

        self.env += match self.phase {
            Phase::Attack => self.attack,
            Phase::Decay1 => -1.0 * self.attack,
            Phase::Decay2 => -0.0000009,
            Phase::Release => -0.0001,
            Phase::Neutural => 0.0,
        };

        self.i += 1;
        self.update_phase();

        self.env
        // if ns > 1.0 {
        //     1.0
        // } else {
        //     ns
        // }
    }

    fn available(&self) -> bool {
        self.env <= 0.0
    }

    fn press(&mut self) {
        self.pressed = true;
        self.phase = Phase::Attack;
        self.i = 0;
        self.env = 0.0;
    }

    fn release(&mut self) {
        self.pressed = false;
        self.phase = Phase::Release;
        self.i = 0;
    }

    fn set_param_1(&mut self, param: Float) {
        let param = param.powi(2) * 2.0 * param;
        self.attack = (param + 0.01) * (1.0 / (self.sample_rate * 0.25));
        // println!("{}", self.attack);
    }

    fn set_param_2(&mut self, param: Float) {
        self.decay = param;
    }

    fn set_param_3(&mut self, _param: Float) {}
}
