pub type Float = f32;
pub type Note = Float;

pub mod echo;
pub mod envelope;
pub mod notes;
pub mod synth;
pub mod trem;
pub mod wave_table_osc;

// any higher and the synth gets angry
#[cfg(not(debug_assertions))]
pub const N_OSCILATORS: usize = 4;
#[cfg(debug_assertions)]
pub const N_OSCILATORS: usize = 3;
