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
const U16_MAX: Float = u16::MAX as Float;
// const FREQ: Float = 440.0;
// const CHORD: [Float; 3] = [FREQ, FREQ * 32.0 / 27.0, FREQ * 3.0 / 2.0];
// const CHORD: [Float; 3] = [164.81, 196.00, 220.00];

fn convert(sample: Float) -> (u8, u8) {
    // (((sample + 1.0) / 2.0) * 255.0) as u8 // let sample = frame.channels()[0].to_f32();
    // (((sample * 0.5) + 0.5) * 255.0) as u8 // let sample = frame.channels()[0].to_f32();
    debug_assert!(sample < 1.0);
    debug_assert!(sample > -1.0);
    let normal = (((sample * 0.5) + 0.5) * U16_MAX) as u16;

    (
        (normal & 0b_0000_0000_1111_1111_u16) as u8,
        (normal >> 8) as u8,
    )
}

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
        let sample = convert(synth.lock().unwrap().get_sample());
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
        // loop {
        for (name, note_len, q_len) in SONG {
            let note = *NOTES.get(name).unwrap();
            // info!("note: {note}, for {note_len} us");
            syn.lock().unwrap().set_frequency(note);
            FreeRtos::delay_us(note_len);
            syn.lock().unwrap().set_frequency(0.0);
            FreeRtos::delay_us(q_len);
            FreeRtos::delay_ms(1);
        }
        // for note in SCALE {
        //     info!("switching note {note}...");
        //     syn.lock().unwrap().set_frequency(note);
        //     FreeRtos::delay_us(250_000);
        // }

        syn.lock().unwrap().set_frequency(0.0);
        info!("*** done ***");

        // FreeRtos::delay_us(1_000_000);
        // }
    })
    .join();

    info!("*** NOW EXITING ***");
    Ok(())
}
