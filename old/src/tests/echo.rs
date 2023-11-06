use super::{CHORD, COMPLETE_MSG};
use crate::{synth::synth::Synth, Float};
use esp_idf_svc::hal::delay::FreeRtos;
use log::info;
use std::sync::{Arc, Mutex};

pub const ECHO_SPEED: Float = 5.0;
pub const ECHO_VOLUME: Float = 0.9;

pub fn test(synth: &Arc<Mutex<Synth>>) {
    info!("*** Echo Effect ***");

    synth.lock().unwrap().echo.set_speed(ECHO_SPEED);
    synth.lock().unwrap().echo.set_volume(ECHO_VOLUME);
    synth.lock().unwrap().echo(true);

    for _ in 0..2 {
        for note in CHORD {
            synth.lock().unwrap().play(note);
        }

        FreeRtos::delay_us(250_000);

        for note in CHORD {
            synth.lock().unwrap().stop(note);
        }

        FreeRtos::delay_us(250_000);
    }

    FreeRtos::delay_us(3_000_000);

    synth.lock().unwrap().echo(false);

    info!("{COMPLETE_MSG}");
}
