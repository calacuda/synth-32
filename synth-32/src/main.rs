use anyhow::{bail, Result};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay::{FreeRtos, BLOCK};
use esp_idf_svc::hal::i2s::{config, I2sDriver};
use esp_idf_svc::hal::peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use esp_idf_svc::hal::{adc, gpio::*}; // {AnyIOPin, AnyOutputPin, PinDriver};
use log::*;
use std::sync::{Arc, Mutex};
use std::thread;
use synth::{synth::Synth, Float};

use crate::controls::effect_conf::GenEffectConf;
use crate::controls::scanner::{Scanner, ADC};
use crate::tests::run_test;

mod controls;
mod notes;
mod tests;

pub type Note = Float;

const SAMPLE_RATE: u32 = 44_100;
// const SAMPLE_RATE: u32 = 38_750;

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
    let config = config::StdConfig::philips(SAMPLE_RATE, config::DataBitWidth::Bits16);
    let mut i2s = I2sDriver::new_std_tx(
        peripherals.i2s0,
        &config,
        pins.gpio12,              // bclk
        pins.gpio13,              // dout
        Option::<AnyIOPin>::None, // mclk
        pins.gpio14,              // ws (l-r-clk)
    )?;
    i2s.tx_enable()?;

    let wave_table_size = 64;

    let synth = Arc::new(Mutex::new(Synth::new(wave_table_size, SAMPLE_RATE)));

    let mut ctrl = Scanner {
        octave: 2,
        tick_i: 0,
        trem_conf: GenEffectConf::new(),
        echo_conf: GenEffectConf::new(),
        octave_up: false,
        octave_down: false,
        columns: [
            PinDriver::output(<Gpio42 as Into<AnyOutputPin>>::into(pins.gpio42))?,
            PinDriver::output(<Gpio20 as Into<AnyOutputPin>>::into(pins.gpio20))?,
            PinDriver::output(<Gpio48 as Into<AnyOutputPin>>::into(pins.gpio48))?,
            PinDriver::output(<Gpio45 as Into<AnyOutputPin>>::into(pins.gpio45))?,
            PinDriver::output(<Gpio35 as Into<AnyOutputPin>>::into(pins.gpio35))?,
            PinDriver::output(<Gpio36 as Into<AnyOutputPin>>::into(pins.gpio36))?,
            PinDriver::output(<Gpio38 as Into<AnyOutputPin>>::into(pins.gpio38))?,
            PinDriver::output(<Gpio39 as Into<AnyOutputPin>>::into(pins.gpio39))?,
        ],
        rows: [
            PinDriver::input(<Gpio41 as Into<AnyIOPin>>::into(pins.gpio41))?,
            PinDriver::input(<Gpio47 as Into<AnyIOPin>>::into(pins.gpio47))?,
            PinDriver::input(<Gpio40 as Into<AnyIOPin>>::into(pins.gpio40))?,
            PinDriver::input(<Gpio37 as Into<AnyIOPin>>::into(pins.gpio37))?,
        ],
        adc: ADC {
            vol: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio4)?,
            driver: adc::AdcDriver::new(
                peripherals.adc1,
                &adc::config::Config::new().calibration(true),
            )?,
        },
        synth: synth.clone(),
        vol: 1.0,
    };

    ctrl.set_pull()?;

    ThreadSpawnConfiguration {
        name: Some("audio-playback\0".as_bytes()),
        pin_to_core: Some(Core::Core1),
        ..Default::default()
    }
    .set()?;

    // let syn = synth.clone();
    thread::spawn(move || loop {
        let sample = synth.lock().unwrap().get_sample();
        // println!("{sample:?}");
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
        // run_test(&ctrl.synth);
        // ctrl.synth.lock().unwrap().echo(true);
        ctrl.synth.lock().unwrap().echo.set_speed(0.25);
        ctrl.synth.lock().unwrap().echo.set_volume(0.9);
        ctrl.synth.lock().unwrap().set_trem_freq(3.0);
        ctrl.synth.lock().unwrap().set_trem_depth(0.75);

        loop {
            if let Err(e) = ctrl.step() {
                error!("{e}");
            }
            FreeRtos::delay_ms(1);
        }
    })
    .join();

    // loop {}
    info!("*** TESTS COMPLETE ***");
    info!("*** NOW EXITING ***");
    Ok(())
}
