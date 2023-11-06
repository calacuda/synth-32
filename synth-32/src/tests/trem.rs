use super::{CHORD, COMPLETE_MSG};
use crate::{synth::synth::Synth, Float};
use esp_idf_svc::hal::delay::FreeRtos;
use log::info;
use std::sync::{Arc, Mutex};

pub const TREM_SPEED: Float = 20.0;
pub const TREM_DEPTH: Float = 0.5;

pub fn test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Tremolo ***");

    for note in CHORD {
        synth.lock().unwrap().play(note);
    }

    FreeRtos::delay_us(1_000_000);

    synth.lock().unwrap().set_trem_freq(TREM_SPEED);
    synth.lock().unwrap().set_trem_depth(TREM_DEPTH);
    synth.lock().unwrap().tremolo(true);
    info!("starting tremolo");

    FreeRtos::delay_us(4_000_000);

    for note in CHORD {
        synth.lock().unwrap().stop(note);
    }

    synth.lock().unwrap().tremolo(false);

    info!("{COMPLETE_MSG}");
}
