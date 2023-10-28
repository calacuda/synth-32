use anyhow::{bail, Result};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::delay::BLOCK;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::i2s::{config, I2sDriver};
use esp_idf_svc::hal::peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use log::*;
use moanin::SONG;
use notes::NOTES;
use std::sync::{Arc, Mutex};
use std::thread;
use synth::{synth::Synth, Float};

mod moanin;
mod notes;
mod synth;

pub type Note = Float;

const SAMPLE_RATE: u32 = 44_100;
const SCALE: [Note; 8] = [
    261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
];
const CHORD: [Float; 3] = [164.81, 196.00, 220.00];

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("taking peripherals...");
    let peripherals = match peripherals::Peripherals::take() {
        Ok(periph) => periph,
        Err(e) => {
            error!("peripherals not taken");
            bail!("Peripheral could not be taken: {e}");
        }
    };
    let pins = peripherals.pins;
    info!("peripherals acquired...");

    let config = config::StdConfig::philips(SAMPLE_RATE, config::DataBitWidth::Bits16);
    let mut i2s = I2sDriver::new_std_tx(
        peripherals.i2s0,
        &config,
        pins.gpio18,              // bclk
        pins.gpio5,               // dout
        Option::<AnyIOPin>::None, // mclk
        pins.gpio19,              // ws (l-r-clk)
    )?;
    i2s.tx_enable()?;

    let wave_table_size = 64;

    let synth = Arc::new(Mutex::new(Synth::new(wave_table_size, SAMPLE_RATE)));

    ThreadSpawnConfiguration {
        name: Some("audio-playback\0".as_bytes()),
        pin_to_core: Some(Core::Core1),
        ..Default::default()
    }
    .set()?;

    let syn = synth.clone();
    thread::spawn(move || loop {
        let sample = synth.lock().unwrap().get_sample();
        if let Err(why) = i2s.write(&[sample.0, sample.1, sample.0, sample.1], BLOCK) {
            error!("could not send data bc {why}");
        }
    });

    ThreadSpawnConfiguration {
        name: Some("change-notes\0".as_bytes()),
        pin_to_core: Some(Core::Core0),
        ..Default::default()
    }
    .set()?;

    let _ = thread::spawn(move || {
        info!("*** playing scale ***");

        for note in SCALE {
            syn.lock().unwrap().play(note);
            FreeRtos::delay_us(250_000);
            syn.lock().unwrap().stop(note);
        }

        info!("*** done ***");

        FreeRtos::delay_us(1_000_000);

        info!("*** playing song ***");

        // for _ in 0..2 {
        for (name, note_len, q_len) in SONG {
            let note = *NOTES.get(name).unwrap();
            syn.lock().unwrap().play(note);
            FreeRtos::delay_us(note_len);
            syn.lock().unwrap().stop(note);
            FreeRtos::delay_us(q_len);
            FreeRtos::delay_ms(1);
        }
        // }

        info!("*** done ***");

        FreeRtos::delay_us(1_000_000);

        info!("*** arpegeo ***");

        for note in CHORD {
            syn.lock().unwrap().play(note);
            FreeRtos::delay_us(250_000);
            syn.lock().unwrap().stop(note);
        }

        info!("*** done ***");

        FreeRtos::delay_us(1_000_000);

        info!("*** rolling chord ***");

        for note in CHORD {
            syn.lock().unwrap().play(note);
            FreeRtos::delay_us(250_000);
        }

        FreeRtos::delay_us(1_000_000);

        for note in CHORD {
            syn.lock().unwrap().stop(note);
            FreeRtos::delay_us(250_000);
        }
        info!("*** done ***");

        FreeRtos::delay_us(1_000_000);

        info!("*** tremolo ***");

        for note in CHORD {
            syn.lock().unwrap().play(note);
        }

        FreeRtos::delay_us(1_000_000);

        syn.lock().unwrap().set_trem_freq(0.8);
        syn.lock().unwrap().set_trem_depth(0.75);
        syn.lock().unwrap().tremolo(true);

        FreeRtos::delay_us(3_000_000);

        for note in CHORD {
            syn.lock().unwrap().stop(note);
        }

        info!("*** done ***");
    })
    .join();

    info!("*** tests complete ***");
    info!("*** NOW EXITING ***");
    Ok(())
}
