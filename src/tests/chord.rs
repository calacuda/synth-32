use super::{CHORD, COMPLETE_MSG};
use crate::synth::synth::Synth;
use esp_idf_svc::hal::delay::FreeRtos;
use log::info;
use std::sync::{Arc, Mutex};

pub fn test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Rolling Chord ***");

    for note in CHORD {
        synth.lock().unwrap().play(note);
        FreeRtos::delay_us(125_000);
    }

    FreeRtos::delay_us(100_000);

    for note in CHORD {
        synth.lock().unwrap().stop(note);
        FreeRtos::delay_us(125_000);
    }

    info!("{COMPLETE_MSG}");
}
