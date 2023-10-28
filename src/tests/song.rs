use super::COMPLETE_MSG;
use crate::{notes::NOTES, synth::synth::Synth};
use esp_idf_svc::hal::delay::FreeRtos;
use log::info;
use std::sync::{Arc, Mutex};

const BEAT: f64 = 1_000_000.0;

lazy_static::lazy_static! {
    static ref MOANIN: [(Vec<&'static str>, u32, u32); 20] = [
        // Phrase 1 section 1
        (vec!["F3"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["F3"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["Ab3"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["Ab3"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["F3"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["C3"], (BEAT / 8.0) as u32, (BEAT / 16.0) as u32),
        (vec!["Eb3"], (BEAT / 2.0) as u32, (BEAT / 16.0) as u32),
        (vec!["F3"], (BEAT / 4.0) as u32, (BEAT / 4.0) as u32),
        // Chords
        (
            vec!["F4", "A#4", "D5"],
            ((BEAT / 4.0) * 3.0) as u32,
            (BEAT / 32.0) as u32,
        ),
        (
            vec!["F4", "A4", "C5"],
            (BEAT / 2.0) as u32,
            (BEAT / 4.0) as u32,
        ),
        // Phrase 1 section 2
        (vec!["F3"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["F3"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["Ab3"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["C4"], (BEAT / 4.0) as u32, (BEAT / 16.0) as u32),
        (vec!["C4"], (BEAT / 16.0) as u32, (BEAT / 32.0) as u32),
        (vec!["C#4"], (BEAT / 16.0) as u32, (BEAT / 32.0) as u32),
        (vec!["C4"], (BEAT / 16.0) as u32, (BEAT / 32.0) as u32),
        (vec!["A#3"], (BEAT / 4.0) as u32, (BEAT / 4.0) as u32),
        // Chords
        (
            vec!["F4", "A#4", "D5"],
            ((BEAT / 4.0) * 3.0) as u32,
            (BEAT / 32.0) as u32,
        ),
        (
            vec!["F4", "A4", "C5"],
            (BEAT / 2.0) as u32,
            (BEAT / 4.0) as u32,
        ),
    ];
}

pub fn test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Playing Song ***");

    // for _ in 0..2 {
    for (names, note_len, q_len) in MOANIN.iter() {
        let notes = names.iter().map(|name| NOTES.get(name).unwrap());
        for note in notes.clone() {
            synth.lock().unwrap().play(*note);
        }
        FreeRtos::delay_us(*note_len);
        for note in notes {
            synth.lock().unwrap().stop(*note);
        }
        FreeRtos::delay_us(*q_len);
        FreeRtos::delay_ms(1);
    }
    // }

    info!("{COMPLETE_MSG}");
}
