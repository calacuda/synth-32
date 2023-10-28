use anyhow::{bail, Result};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay::BLOCK;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::i2s::{config, I2sDriver};
use esp_idf_svc::hal::peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use log::*;
use std::sync::{Arc, Mutex};
use std::thread;
use synth::{synth::Synth, Float};

use crate::tests::run_test;

mod notes;
mod synth;
mod tests;

pub type Note = Float;

const SAMPLE_RATE: u32 = 44_100;

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
        info!("*** RUNNING TESTS ***");
        run_test(&syn);
        info!("*** Done ***");
    })
    .join();

    info!("*** TESTS COMPLETE ***");
    info!("*** NOW EXITING ***");
    Ok(())
}
