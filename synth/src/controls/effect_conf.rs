use super::Float;

#[derive(Debug, PartialEq, Eq)]
pub enum ButtonState {
    /// the button is currently depressed
    Pressed,
    /// the button was just released
    Released,
    /// the button is in a neutural possision and has been for 1 or more ticks
    Dormant,
}

impl ButtonState {
    pub fn cycle(&mut self) -> ButtonState {
        match self {
            ButtonState::Pressed => ButtonState::Released,
            ButtonState::Released => ButtonState::Dormant,
            ButtonState::Dormant => ButtonState::Pressed,
        }
    }
}

pub struct GenEffectConf {
    // pub button_state: ButtonState,
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

    // pub fn cycle(&mut self) -> bool {
    //     self.button_state = self.button_state.cycle();
    //     self.button_state == ButtonState::Released
    // }
}
