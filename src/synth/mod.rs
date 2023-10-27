pub type Float = f32;

pub mod synth;
mod wave_table_osc;

// any higher and the synth gets angry
#[cfg(not(debug_assertions))]
pub const POLYPHONY: usize = 6;
#[cfg(debug_assertions)]
pub const POLYPHONY: usize = 4;
