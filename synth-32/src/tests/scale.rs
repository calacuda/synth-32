use super::COMPLETE_MSG;
use crate::Note;
use esp_idf_svc::hal::delay::FreeRtos;
use log::info;
use std::sync::{Arc, Mutex};
use synth::synth::Synth;

const SCALE: [Note; 8] = [
    261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
];

pub fn test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Playing Scale ***");

    for note in SCALE {
        synth.lock().unwrap().play(note);
        FreeRtos::delay_us(250_000);
        synth.lock().unwrap().stop(note);
    }

    info!("{COMPLETE_MSG}");
}
