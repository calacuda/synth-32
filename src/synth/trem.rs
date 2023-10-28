use super::wave_table_osc::WavetableOscillator;
use super::Float;

pub struct Tremolo {
    pub on: bool,
    pub depth: Float,
    pub osc: WavetableOscillator,
}

impl Tremolo {
    pub fn new(osc: WavetableOscillator) -> Self {
        Self {
            on: false,
            depth: 0.5,
            osc,
        }
    }

    pub fn set_status(&mut self, status: bool) {
        self.on = status;
    }

    pub fn get_sample(&mut self) -> Float {
        self.osc.get_sample() * self.depth
    }
}
