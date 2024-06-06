pub type Float = f32;
pub type Note = Float;

pub mod echo;
pub mod envelope;
pub mod lowpass;
pub mod notes;
pub mod osc;
pub mod synth;
pub mod trem;
pub mod wave_table_osc;

// any higher and the synth gets angry
// TODO: tweak these values
#[cfg(not(debug_assertions))]
pub const N_OSCILATORS: usize = 4;
#[cfg(debug_assertions)]
pub const N_OSCILATORS: usize = 3;

#[cfg(not(debug_assertions))]
pub const DISCOUNT: Float = 2.0;
#[cfg(debug_assertions)]
pub const DISCOUNT: Float = 1.73205081;

#[cfg(not(debug_assertions))]
pub const N_OVERTONES: usize = 15;
#[cfg(debug_assertions)]
pub const N_OVERTONES: usize = 6;

// pub const SAMPLE_RATE: u32 = 44_100;
pub const SAMPLE_RATE: u32 = 22_050;
// pub const SAMPLE_RATE: u32 = 24_000;
