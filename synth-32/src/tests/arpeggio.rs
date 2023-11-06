use super::{CHORD, COMPLETE_MSG};
use crate::synth::synth::Synth;
use esp_idf_svc::hal::delay::FreeRtos;
use log::info;
use std::sync::{Arc, Mutex};

pub fn test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Arpeggio ***");

    for note in CHORD {
        synth.lock().unwrap().play(note);
        FreeRtos::delay_us(250_000);
        synth.lock().unwrap().stop(note);
    }

    info!("{COMPLETE_MSG}");
}
