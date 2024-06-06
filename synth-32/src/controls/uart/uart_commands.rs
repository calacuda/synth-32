use crate::Float;
use serde::{Deserialize, Serialize};

/// the uart command struct that is received from the pico
#[derive(Debug, Serialize, Deserialize)]
pub struct UartCommand {
    /// what kind of data is being accessed/modified by a command
    pub entity: Entity,
    /// will be true if the command is "setting" the value and false if "getting" the value.
    pub set: bool,
    /// command specific argument
    pub args: Option<Float>,
}

/// what kind of data is being accessed/modified by a command
#[derive(Debug, Serialize, Deserialize)]
pub enum Entity {
    Volume,
    EnvAttack,
    EnvDecay,
    EnvSustain,
    LowPassCutoff,
    LowPassResonance,
    PlayNote,
    StopNote,
    BendNote,
    DelaySpeed,
    DelayVolume,
}
