use crate::{Float, SAMPLE_RATE};
use log::{error, info};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Phase {
    Neutural,
    Attack,
    Decay,
    Sustain,
    Release,
}

#[derive(Debug, Clone, Copy)]
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
    // release_threshold: Float,
    i: Float,
}

impl Filter {
    pub fn new() -> Self {
        let i = 0.0;
        let sample_rate = SAMPLE_RATE as Float;
        let attack_speed = 0.01;
        // let attack = 1.0 / (sample_rate * attack_speed);
        let attack = get_slope((0.0, 1.0), (0.0, (sample_rate * attack_speed)));
        let decay_speed = 0.1;
        // let decay = -1.0 / (sample_rate * decay_speed);
        let threshold = 0.25;
        let decay = get_slope((1.0, threshold), (0.0, (sample_rate * decay_speed)));

        Self {
            // pressed: false,
            phase: Phase::Neutural,
            // i: 0,
            env: 0.0,
            decay,
            attack,
            threshold,
            sample_rate,
            attack_speed,
            decay_speed,
            release: get_slope((threshold, 0.0), (0.0, (sample_rate * 0.01))),
            pressed: false,
            // release_threshold: 0.05,
            i,
        }
    }

    pub fn set_attack(&mut self, atk_speed: Float) {
        let atk_speed = 1.0 - atk_speed;

        if atk_speed != self.attack_speed {
            self.attack_speed = atk_speed;
            // info!("setting attack to {atk_speed}");
            // self.attack = 1.0 / (self.sample_rate * atk_speed);
            self.attack = get_slope((0.0, 1.0), (0.0, (self.sample_rate * atk_speed)));
            // info!("setting attack_env to {}", self.attack);
        }
    }

    pub fn set_decay(&mut self, decay_speed: Float) {
        let decay_speed = 1.0 - decay_speed;

        if decay_speed != self.decay_speed {
            self.decay_speed = decay_speed;
            // info!("setting decay to {decay_speed}");
            // self.decay = -1.0 / (self.sample_rate * decay_speed);
            self.decay = get_slope(
                (1.0, self.threshold),
                (0.0, (self.sample_rate * decay_speed)),
            );
            // info!("setting decay_env to {}", self.decay);
        }
    }

    pub fn set_threshold(&mut self, threshold: Float) {
        if threshold != self.threshold {
            self.threshold = threshold;
            // info!("setting sustain to {threshold}");
            // self.release = -threshold / (self.sample_rate * 0.001);
            self.release = get_slope((threshold, 0.0), (0.0, (self.sample_rate * 0.01)));
            self.decay = get_slope(
                (1.0, self.threshold),
                (0.0, (self.sample_rate * self.decay_speed)),
            );
            // info!("setting release to {}", self.release);
        }
    }

    pub fn set_sustain(&mut self, sustain: Float) {
        self.set_threshold(sustain);
    }

    fn internal_update_phase(&mut self) {
        if self.phase == Phase::Attack && self.env >= 1.0 {
            self.phase = Phase::Decay;
            // info!("changing phase to => {:?} => {}", self.phase, self.env);
            self.env = 1.0;
        } else if self.phase == Phase::Decay && self.env <= self.threshold {
            self.phase = Phase::Sustain;
            // info!("changing phase to => {:?} => {}", self.phase, self.env);
            // } else if self.phase == Phase::Sustain && self.env <= self.release {
            //     self.phase = Phase::Release;
            //     info!("changing phase to => {:?}", self.phase);
        } else if self.phase == Phase::Release && self.env <= 0.0 {
            self.phase = Phase::Neutural;
            // info!("changing phase to => {:?} => {}", self.phase, self.env);
            self.env = 0.0;
        }

        if self.env == Float::INFINITY {
            error!("env reached INF!");
            self.env = 1.0;
        } else if self.env == Float::NEG_INFINITY {
            error!("env reached NEG_INF!");
            self.env = 0.0;
        }
    }

    pub fn release(&mut self) {
        self.phase = Phase::Release;
        self.pressed = false;
        self.i = 0.0;
        // info!("on release self.env = {}", self.get_env());
    }

    pub fn press(&mut self) {
        self.phase = Phase::Attack;
        self.pressed = true;
        self.i = 0.0;
        // info!("on press self.env = {}", self.get_env());
    }

    pub fn get_env(&mut self) -> Float {
        self.env
    }

    // fn set_env(&mut self, env: Float) {
    //     self.env = env;
    // }

    fn get_step(&mut self) -> Float {
        match self.phase {
            // Phase::Attack => (1.0 / self.sample_rate).powf(1.0 - self.i * self.attack_speed), // self.attack,
            Phase::Attack => self.env + self.attack,
            // Phase::Decay => {
            // ((1.0 / self.sample_rate) + self.threshold).powf(self.i * self.decay_speed)
            // } // self.decay,
            Phase::Decay => self.env + self.decay,
            Phase::Sustain => self.env,
            Phase::Release => self.env + self.release,
            Phase::Neutural => {
                // info!("in Neutural state, env value = {}", self.env);
                0.0
            }
        }
    }

    fn update_phase(&mut self) {
        self.internal_update_phase()
    }

    // fn open_filter(&mut self, samples: Vec<Float>) -> bool {
    //     let sample: Float = samples.iter().sum::<Float>().tanh();
    //
    //     if self.pressed && sample <= 0.75 {
    //         // info!("release");
    //         self.phase = Phase::Release;
    //         self.pressed = false;
    //     } else if !self.pressed
    //         && (self.phase == Phase::Neutural || self.phase == Phase::Release)
    //         && sample >= 0.75
    //     {
    //         // info!("pressed");
    //         self.phase = Phase::Attack;
    //         self.pressed = true;
    //     }
    //
    //     self.pressed
    // }

    // fn take_input(&mut self, input: u8, samples: Vec<Float>) -> Result<()> {
    //     let sample: Float = samples.iter().sum::<Float>();
    //
    //     match input {
    //         // attack in
    //         0 => self.set_attack(sample),
    //         // decay speed in
    //         1 => self.set_decay(sample),
    //         // decay_threshold in
    //         2 => self.set_threshold(sample),
    //         n => bail!("{n} is not a valid input for the ADSR filter."),
    //     }
    //
    //     Ok(())
    // }

    // fn pressed(&mut self) -> bool {
    //     self.phase != Phase::Neutural
    //     // self.pressed
    // }

    pub fn envelope(&mut self) -> Float {
        self.step_env();
        let env = self.get_env();
        self.update_phase();

        env
    }

    fn step_env(&mut self) {
        // let new_env = self.get_env() + self.get_step();
        // // self.set_env(new_env);
        // self.env = new_env;
        let step = self.get_step();
        // info!("env = {}, step = {step}", self.env);
        self.env = step;
    }

    pub fn available(self) -> bool {
        !self.pressed
    }

    pub fn get_envelope(&mut self) -> Float {
        self.step_env();

        // println!("{:?} => {}", self.phase, self.env);
        // let env = self.get_env();
        // info!("adsr phase => {:?}, adsr env => {env}", self.phase);
        self.i += 1.0;
        self.i %= self.sample_rate;
        self.update_phase();

        // println!("{}", self.env);
        self.env
    }
}

fn get_slope(ys: (Float, Float), xs: (Float, Float)) -> Float {
    (ys.0 - ys.1) / (xs.0 - xs.1)
}

// fn get_slope_curve(ys: (Float, Float), xs: (Float, Float)) -> Float {
//     (ys.0 - ys.1) / (xs.0 - xs.1)
// }
