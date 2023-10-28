use crate::{synth::synth::Synth, Float};
use esp_idf_svc::hal::delay::FreeRtos;
use std::sync::{Arc, Mutex};

mod all_notes;
mod arpeggio;
mod chord;
mod scale;
mod song;
mod trem;

pub const COMPLETE_MSG: &str = "*** Done ***";
pub const CHORD: [Float; 3] = [164.81, 196.00, 220.00];

pub fn run_test(synth: &Arc<Mutex<Synth>>) {
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
    trem::test(synth);
    FreeRtos::delay_us(1_000_000);
    // echo/delay
    //
    // FreeRtos::delay_us(1_000_000);
    // echo/delay & trem
    //
    // FreeRtos::delay_us(1_000_000);
    // all notes (dry)
    all_notes::test(synth);
    FreeRtos::delay_us(1_000_000);
    // all notes (trem)
    all_notes::trem_test(synth);
    // FreeRtos::delay_us(1_000_000);
}
