use anyhow::{bail, Result};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay::{BLOCK, NON_BLOCK};
use esp_idf_svc::hal::i2s::{config, I2sDriver};
use esp_idf_svc::hal::peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use esp_idf_svc::hal::uart::config::Config;
use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_svc::hal::{adc, gpio::*};
use esp_idf_svc::io::Read;
use esp_idf_svc::hal::i2s::config::Role;
use esp_idf_svc::espnow::EspNow;
use log::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::sync::mpsc::sync_channel;
use synth::{Float, wave_table_osc::WavetableOscillator, SAMPLE_RATE};

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
    // let config = config::StdConfig::philips(SAMPLE_RATE, config::DataBitWidth::Bits32); // maybe
    let config_out = config::StdConfig::philips(SAMPLE_RATE, config::DataBitWidth::Bits32); // maybe
    // let config_out = config::StdConfig::new(esp_idf_svc::hal::i2s::config::Config::new()); // maybe
    let config_in = config::StdConfig::new(
        esp_idf_svc::hal::i2s::config::Config::new().role(Role::Target), 
        esp_idf_svc::hal::i2s::config::StdClkConfig::from_sample_rate_hz(SAMPLE_RATE),
        esp_idf_svc::hal::i2s::config::StdSlotConfig::philips_slot_default(config::DataBitWidth::Bits32, esp_idf_svc::hal::i2s::config::SlotMode::Mono),
        esp_idf_svc::hal::i2s::config::StdGpioConfig::default(),
    ); // maybe
    // change to 16;

    info!("building i2s out");
    let mut i2s_out = I2sDriver::new_std_tx(
        peripherals.i2s0,
        &config_out,
        pins.gpio12,              // bclk
        pins.gpio13,              // d-out
        Option::<AnyIOPin>::None, // mclk
        pins.gpio14,              // ws (l-r-clk)
    )?;
    i2s_out.tx_enable()?;

    // info!("building ESP_NOW");
    // // TODO: build ESP_NOW struct
    info!("building I2S in");
    let mut i2s_in = I2sDriver::new_std_rx(
        peripherals.i2s1,
        &config_in,
        pins.gpio25,              // bclk
        pins.gpio26,              // d-out
        Option::<AnyIOPin>::None, // mclk
        // Some(pins.gpio0),
        pins.gpio27,              // ws (l-r-clk)
    )?;
    i2s_in.rx_enable()?;

    info!("building I2C");
    // TODO: build I2C struct

    info!("waiting for I2C scan...");
    // TODO: wait for I2C scan and respond with module type and MAC addr.
    
    info!("building lowpass filter");
    let mut filter = synth::lowpass::LowPassFilter::new();

    info!("starting play loop");
    // TODO: make this a generic function that takes i2s_in, i2s_out, i2c_in, and a closure of how
    // to generate new samples.

    let (tx, rx) = crossbeam::channel::bounded(1);

    // let mut in_sample = 0.0;

    spawn(
        move || {
            loop {
                let mut in_bytes = [0; 8];
                if let Err(e) = i2s_in.read(&mut in_bytes, BLOCK) {
                    error!("failed to read i2S in. {e}");
                }
                let in_sample_bytes: [u8; 4] = [in_bytes[0], in_bytes[1], in_bytes[2], in_bytes[3]];
                let in_sample = Float::from_le_bytes(in_sample_bytes);
                tx.send(in_sample);
            }
        }
    );

    // spawn(
    //     move || {
    //         loop {
    //             let mut in_bytes = [0; 8];
    //             i2s_in.read(&mut in_bytes, BLOCK)?;
    //             let in_sample_bytes: [u8; 4] = [in_bytes[0], in_bytes[1], in_bytes[2], in_bytes[3]];
    //             in_sample = Float::from_le_bytes(in_sample_bytes);
    //             tx.send(in_sample);
    //         }
    //     }
    // );

    loop {
        // // check for new i2s in data
        // let mut in_bytes = [0; 8];
        // // let filter_sample = if generate {
        // let filter_sample = filter.get_sample(in_sample);
        // // } else {}
        //     // 0.0
        // // };
        //
        // i2s_in.read(&mut in_bytes, BLOCK)?;
        // let start = 0;
        // let in_sample_bytes: [u8; 4] = [in_bytes[start + 0], in_bytes[start + 1], in_bytes[start + 2], in_bytes[start + 3]];
        // in_sample = Float::from_le_bytes(in_sample_bytes);
        // if in_sample > 0.0 {
        //     generate = true;
        // }
        // info!("in_sample => {in_sample}");
        // info!("samples : {in_bytes:?} => {in_sample} => {filter_sample}");
        // info!("samples : {in_bytes:?} => {in_sample}");
        // filter.take_sample(in_sample);
        // TODO: check for new i2c control data

        let in_sample = rx.recv().unwrap_or(0.0);
        // let filter_sample = filter.get_sample(in_sample);
        // generate sample
        // let sample = filter_sample.to_le_bytes();
        let sample = in_sample.to_le_bytes();
        // send sample
        i2s_out.write(&[sample[0], sample[1], sample[2], sample[3], sample[0], sample[1], sample[2], sample[3]], BLOCK)?;
    }
}
