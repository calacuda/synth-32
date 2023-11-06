use super::Float;

pub struct GenEffectConf {
    pub pressed: bool,
    pub volume: Float,
    pub speed: Float,
}

impl GenEffectConf {
    pub fn new() -> Self {
        Self {
            // button_state: ButtonState::Dormant,
            pressed: false,
            volume: 0.0,
            speed: 0.0,
        }
    }
}
