pub type Float = f32;

mod echo;
pub mod synth;
mod trem;
mod wave_table_osc;

// any higher and the synth gets angry
#[cfg(not(debug_assertions))]
pub const N_OSCILATORS: usize = 5;
#[cfg(debug_assertions)]
pub const N_OSCILATORS: usize = 4;
