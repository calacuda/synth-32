use crate::Float;
use esp_idf_svc::hal::delay::FreeRtos;
use log::*;
use std::sync::{Arc, Mutex};
use synth::synth::Synth;

// mod all_notes;
mod arpeggio;
mod chord;
// mod echo;
mod scale;
mod song;
// mod trem;

pub const COMPLETE_MSG: &str = "*** Done ***";
pub const CHORD: [Float; 3] = [164.81, 196.00, 220.00];

pub fn run_test(synth: &Arc<Mutex<Synth>>) {
    info!("*** RUNNING TESTS ***");
    // scale
    scale::test(synth);
    FreeRtos::delay_us(1_000_000);
    // song
    song::test(synth);
    FreeRtos::delay_us(1_000_000);
    // arpeggio
    arpeggio::test(synth);
    FreeRtos::delay_us(1_000_000);
    // chord
    chord::test(synth);
    FreeRtos::delay_us(1_000_000);
    // trem
    // trem::test(synth);
    // FreeRtos::delay_us(1_000_000);
    // echo
    // echo::test(synth);
    // FreeRtos::delay_us(1_000_000);
    // all notes (dry)
    // all_notes::test(synth);
    // FreeRtos::delay_us(1_000_000);
    // all notes (trem)
    // all_notes::trem_test(synth);
    // FreeRtos::delay_us(1_000_000);
    // all notes echo
    // all_notes::echo_test(synth);
    // FreeRtos::delay_us(1_000_000);
    // all ntoes echo & trem
    // all_notes::trem_echo_test(synth);
    // FreeRtos::delay_us(1_000_000);
    info!("*** Done ***");
}
