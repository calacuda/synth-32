use log::info;

use super::Envelope;
use crate::Float;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Phase {
    Neutural,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Debug, Clone, Copy)]
pub struct ADSREnvelope {
    pub pressed: bool,
    phase: Phase,
    i: usize,
    env: Float,
    decay: Float,
    attack: Float,
}

impl ADSREnvelope {
    pub fn new() -> Self {
        Self {
            pressed: false,
            phase: Phase::Neutural,
            i: 0,
            env: 0.0,
            decay: 0.0,
            attack: 0.0,
        }
    }

    fn update_phase(&mut self) {
        if self.phase == Phase::Attack && self.env >= 1.0 {
            // info!("1");
            self.phase = Phase::Decay;
            self.env = 1.0;
            self.i = 0;
        } else if self.phase == Phase::Decay && self.env <= self.decay {
            // info!("2");
            self.phase = Phase::Sustain;
            self.i = 0;
            // } else if self.phase == Phase::Decay2 && self.env <= 0.25 {
            //     // info!("3");
            //     self.phase = Phase::Release;
            //     self.i = 0;
        } else if self.phase == Phase::Release && self.env <= 0.0 {
            self.phase = Phase::Neutural;
        }
    }
}

impl Envelope for ADSREnvelope {
    fn envelope(&mut self) -> Float {
        // if !self.available() {
        //     println!("{:?} => i: {}, env: {}", self.phase, self.i, self.env);
        // }

        self.env += match self.phase {
            Phase::Attack => self.attack,
            Phase::Decay => -1.0 * self.attack,
            Phase::Sustain => 0.0,
            Phase::Release => -0.0001,
            Phase::Neutural => return 0.0,
        };

        self.i += 1;
        self.update_phase();

        // if !self.available() {
        //     println!("{}", self.env);
        // }

        self.env
        // if ns > 1.0 {
        //     1.0
        // } else {
        //     ns
        // }
    }

    fn available(&self) -> bool {
        self.phase == Phase::Neutural
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
        self.attack = param;
    }

    fn set_param_2(&mut self, param: Float) {
        self.decay = param;
    }

    fn set_param_3(&mut self, _param: Float) {}
}
