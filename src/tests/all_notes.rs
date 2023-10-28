use super::COMPLETE_MSG;
use crate::{
    notes::{NOTES, NOTE_NAMES},
    synth::synth::Synth,
};
use esp_idf_svc::hal::delay::FreeRtos;
use log::info;
use std::sync::{Arc, Mutex};

fn note_sweep(synth: &Arc<Mutex<Synth>>) {
    for name in NOTE_NAMES {
        let note = *NOTES.get(name).unwrap();
        synth.lock().unwrap().play(note);
        FreeRtos::delay_us(75_000);
        synth.lock().unwrap().stop(note);
    }
}

pub fn test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Testing All Notes (dry -- no effects) ***");
    note_sweep(synth);
    info!("{COMPLETE_MSG}");
}

pub fn trem_test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Testing All Notes (With Tremolo) ***");

    synth.lock().unwrap().set_trem_freq(5.0);
    synth.lock().unwrap().set_trem_depth(1.0);
    synth.lock().unwrap().tremolo(true);

    note_sweep(synth);

    synth.lock().unwrap().tremolo(false);

    info!("{COMPLETE_MSG}");
}
