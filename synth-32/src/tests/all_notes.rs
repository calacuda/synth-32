use super::echo::{ECHO_SPEED, ECHO_VOLUME};
use super::COMPLETE_MSG;
use crate::{
    notes::{NOTES, NOTE_NAMES},
    Float,
};
use esp_idf_svc::hal::delay::FreeRtos;
use log::info;
use std::sync::{Arc, Mutex};
use synth::synth::Synth;

const TREM_SPEED: Float = 0.5;
const TREM_DEPTH: Float = 0.5;

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

    synth.lock().unwrap().set_trem_freq(TREM_SPEED);
    synth.lock().unwrap().set_trem_depth(TREM_DEPTH);
    synth.lock().unwrap().tremolo(true);

    note_sweep(synth);

    synth.lock().unwrap().tremolo(false);

    info!("{COMPLETE_MSG}");
}

pub fn echo_test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Testing All Notes (With Echo) ***");

    synth.lock().unwrap().echo.set_speed(ECHO_SPEED);
    synth.lock().unwrap().echo.set_volume(ECHO_VOLUME);
    synth.lock().unwrap().echo(true);

    note_sweep(synth);

    synth.lock().unwrap().echo(false);

    info!("{COMPLETE_MSG}");
}

pub fn trem_echo_test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Testing All Notes (With Both Echo & Tremolo) ***");

    synth.lock().unwrap().echo.set_speed(ECHO_SPEED);
    synth.lock().unwrap().echo.set_volume(ECHO_VOLUME);
    synth.lock().unwrap().set_trem_freq(TREM_SPEED);
    synth.lock().unwrap().set_trem_depth(TREM_DEPTH);
    synth.lock().unwrap().tremolo(true);
    synth.lock().unwrap().echo(true);

    note_sweep(synth);

    synth.lock().unwrap().tremolo(false);
    synth.lock().unwrap().echo(false);

    info!("{COMPLETE_MSG}");
}
