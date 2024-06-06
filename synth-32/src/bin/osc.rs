use anyhow::{bail, Result};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay::{BLOCK, NON_BLOCK};
use esp_idf_svc::hal::i2s::{config, I2sDriver};
use esp_idf_svc::hal::peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use esp_idf_svc::hal::uart::config::Config;
use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_svc::hal::{adc, gpio::*};
use log::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use synth::{Float, wave_table_osc::WavetableOscillator};
pub use synth_32::SAMPLE_RATE;

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

    // make and init i2s
    let config = config::StdConfig::philips(SAMPLE_RATE, config::DataBitWidth::Bits32); // maybe
    // change to 16;

    info!("sending samples directly to built in DAC");
    let mut i2s_out = I2sDriver::new_std_tx(
        peripherals.i2s0,
        &config,
        pins.gpio12,              // bclk
        pins.gpio13,              // d-out
        Option::<AnyIOPin>::None, // mclk
        pins.gpio14,              // ws (l-r-clk)
    )?;
    i2s_out.tx_enable()?;

    // info!("building ESP_NOW");
    // // TODO: build ESP_NOW struct
    info!("building I2S in");
    // TODO: build I2S input
    let mut i2s_in = I2sDriver::new_std_rx(
        peripherals.i2s1,
        &config,
        pins.gpio18,              // bclk
        pins.gpio19,              // d-out
        Option::<AnyIOPin>::None, // mclk
        pins.gpio21,              // ws (l-r-clk)
    )?;
    i2s_in.rx_enable()?;

    info!("building I2C");
    // TODO: build I2C struct

    info!("waiting for I2C scan...");
    // TODO: wait for I2C scan and respond with module type and MAC addr.
    
    info!("building Oscillator");
    let mut osc = synth::osc::Oscillator::new(64, SAMPLE_RATE);

    // TODO: make this a generic function that takes i2s_in, i2s_out, i2c_in, and a closure of how
    // to generate new samples.
    loop {
        // check for new i2s in data
        let mut in_sample_bytes = [0; 4];
        let osc_sample = osc.get_sample();
        i2s_in.read(&mut in_sample_bytes, BLOCK);
        let in_sample = Float::from_le_bytes(in_sample_bytes);
        // TODO: check for new i2c control data

        // generate sample
        let sample = (osc_sample + in_sample).to_le_bytes();
        // send sample
        i2s_out.write(&[sample[0], sample[1], sample[2], sample[3], sample[0], sample[1], sample[2], sample[3]], NON_BLOCK);
    }
}
