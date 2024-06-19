use super::{Float, SAMPLE_RATE};
// use log::{error, info, trace};
// use std::f32::consts::PI;
// use synfx_dsp::{Biquad, BiquadCoefs};
use fundsp::{
    audionode::{AudioNode, Frame},
    // hacker32::U1,
    moog::Moog,
};
use generic_array::arr;

// pub struct AllPassFilter {
//     cutoff: Float,
//     base_cutoff: Float,
//     dn_1: Float,
//     env: Float,
//     a1: Float,
//     highpass: bool,
//     base_resonance: Float,
//     resonance: Float,
//     wiggle_discount: Float,
// }
//
// impl AllPassFilter {
//     pub fn new() -> Self {
//         let cutoff = 3_000.0;
//         Self {
//             cutoff,
//             base_cutoff: cutoff,
//             dn_1: 0.0,
//             env: 0.0,
//             a1: 0.0,
//             highpass: false,
//             base_resonance: 0.5,
//             resonance: 0.5,
//             wiggle_discount: 0.2,
//         }
//     }
//
//     fn set_a1_coef(&mut self) {
//         let tan = (PI as Float * self.cutoff / SAMPLE_RATE as Float).tan();
//         self.a1 = (tan - 1.0) / (tan + 1.0)
//     }
//
//     pub fn wiggle_cutoff(&mut self, wiggle_amount: Float) {
//         self.cutoff = self.base_cutoff + (5_000.0 * wiggle_amount) + 100.0;
//         // trace!(
//         //     "base_cutoff: {} | cutoff: {}, | wiggle: {}",
//         //     self.base_cutoff,
//         //     self.cutoff,
//         //     wiggle_amount
//         // );
//
//         self.set_a1_coef();
//     }
//
//     pub fn wiggle_resonance(&mut self, wiggle_amount: Float) {
//         self.resonance = self.base_resonance * wiggle_amount;
//
//         if self.resonance > 1.0 {
//             error!("resonance saved from clipping");
//             self.resonance = 1.0;
//         } else {
//             // trace!("resonance => {}", self.resonance);
//         }
//     }
// }

// impl Filter for AllPassFilter {
//     fn init(&mut self) {
//         self.set_a1_coef();
//     }
//
//     fn take_env(&mut self, env: Float) {
//         self.env = env;
//         // self.wiggle_resonance(env);
//         // self.wiggle_cutoff(env * 0.25);
//     }
//
//     fn get_sample(&mut self, audio_in: Float) -> Float {
//         let result = self.a1 * audio_in + self.dn_1 * self.resonance;
//         // self.dn_1 = audio_in - self.a1 * (result * self.resonance);
//         self.dn_1 = audio_in - self.a1 * result;
//
//         (if self.highpass {
//             result * -0.5
//         } else {
//             result * 0.5
//         })
//         .tanh()
//     }
//
//     /// takes a number between 0 and 1.0
//     fn set_cutoff(&mut self, cutoff: Float) {
//         // trace!("input cutoff: {cutoff}");
//         let cutoff = (cutoff * 5_000.0) + 100.0;
//         self.cutoff = cutoff;
//         self.base_cutoff = cutoff;
//         // trace!("set cutoff to: {}", self.cutoff);
//         self.set_a1_coef();
//     }
//
//     fn set_resonance(&mut self, resonance: Float) {
//         let res = resonance;
//
//         self.resonance = res;
//         self.base_resonance = res;
//     }
// }

type U1 = numeric_array::typenum::U1;

#[derive(Clone)]
pub struct LowPassFilter {
    cutoff: Float,
    base_cutoff: Float,
    resonance: Float,
    base_resonance: Float,
    // env: Float,
    // filter: Biquad,
    filter: Moog<Float, Float, U1>,
}

impl LowPassFilter {
    pub fn new() -> Self {
        // let low_pass_settings = BiquadCoefs::lowpass(SAMPLE_RATE as f32, 0.5, 3_000.0);
        // let mut filter = Biquad::new();
        // filter.set_coefs(low_pass_settings);
        let start_cutoff = 3_000.0;
        let start_res = 0.0;
        let filter = Moog::new(SAMPLE_RATE as f64, start_cutoff, start_res);

        Self {
            cutoff: start_cutoff,
            base_cutoff: start_cutoff,
            resonance: start_res,
            base_resonance: start_res,
            // env: 0.0,
            filter,
        }
    }

    pub fn wiggle_cutoff(&mut self, wiggle_amount: Float) {
        self.cutoff = self.base_cutoff + (5_000.0 * wiggle_amount);
        // trace!(
        //     "base_cutoff: {} | cutoff: {}, | wiggle: {}",
        //     self.base_cutoff,
        //     self.cutoff,
        //     wiggle_amount
        // );

        self.recalculate();
    }

    fn recalculate(&mut self) {
        // info!(
        //     "using cutoff: {}, and resonance: {}.",
        //     self.cutoff, self.resonance
        // );

        // let low_pass_settings = BiquadCoefs::lowpass(
        //     SAMPLE_RATE as f32,
        //     self.resonance as f32,
        //     self.cutoff as f32,
        // );
        // let mut filter = Biquad::new();
        // filter.set_coefs(low_pass_settings);
        //
        // self.filter = filter;

        self.filter.set_cutoff_q(self.cutoff, self.resonance);
    }

    pub fn init(&mut self) {}

    // pub fn take_env(&mut self, env: Float) {
    //     self.env = env;
    //     // maybe change to minus
    //     // self.resonance = (env * 2.0 - 1.0);
    //     // self.wiggle_cutoff(env);
    //     // info!("resonance => {}", self.resonance);
    //     // self.recalculate();
    // }

    pub fn get_sample(&mut self, audio_in: Float) -> Float {
        let ar = arr![Float; audio_in];
        let frame: Frame<Float, U1> = Frame::new(ar);
        self.filter.tick(&frame)[0]
    }

    pub fn set_cutoff(&mut self, cutoff: Float) {
        // let cutoff = cutoff * 5_000.0;
        let cutoff = cutoff * 5_000.0;

        self.cutoff = cutoff;
        self.base_cutoff = cutoff;
        self.recalculate();
    }

    pub fn set_resonance(&mut self, resonance: Float) {
        // let scaler = 19_950.0;
        let res = resonance;
        // info!("{resonance} => {res}");

        self.resonance = res;
        self.base_resonance = res;
        self.recalculate();
    }
}
