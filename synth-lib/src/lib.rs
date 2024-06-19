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
pub const N_OSCILATORS: usize = 7;
#[cfg(debug_assertions)]
pub const N_OSCILATORS: usize = 1;

#[cfg(not(debug_assertions))]
pub const DISCOUNT: Float = 1.0;
#[cfg(debug_assertions)]
pub const DISCOUNT: Float = 1.0 / 2.64575131106;
// pub const DISCOUNT: Float = 1.0 / 1.41421356237;
// pub const DISCOUNT: Float = 1.73205081;

#[cfg(not(debug_assertions))]
// pub const N_OVERTONES: usize = 1;
// pub const N_OVERTONES: usize = 9;
pub const N_OVERTONES: usize = 100;
#[cfg(debug_assertions)]
pub const N_OVERTONES: usize = 3;

// pub const SAMPLE_RATE: u32 = 44_100;
// pub const SAMPLE_RATE: u32 = 22_050;
// pub const SAMPLE_RATE: u32 = 24_000;
pub const SAMPLE_RATE: u32 = 48_000;
