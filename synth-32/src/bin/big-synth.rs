use anyhow::{bail, Result};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay::BLOCK;
use esp_idf_svc::hal::i2s::{config, I2sDriver};
use esp_idf_svc::hal::peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use esp_idf_svc::hal::uart::UartDriver;
use esp_idf_svc::hal::{adc, gpio::*};
use log::*;
// use std::collections::HashMap;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use std::thread;
use synth::{synth::Synth, Float};
use synth_32::controls;
// use synth_32::tests;
pub use synth_32::SAMPLE_RATE;

use crate::controls::effect_conf::GenEffectConf;
use crate::controls::scanner::{Buttons, Scanner, ADC};
use crate::controls::uart;

// mod controls;
// // mod notes;
// mod tests;

// pub type Note = Float;
//
// // const SAMPLE_RATE: u32 = 44_100;
// const SAMPLE_RATE: u32 = 22_050;
// // const SAMPLE_RATE: u32 = 38_750;
//
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
    // let config = config::StdConfig::philips(SAMPLE_RATE, config::DataBitWidth::Bits32);

    // if no peripherals are detected send data directly to the built in DAC
    info!("creating I2S output");
    let mut i2s_out = I2sDriver::new_std_tx(
            peripherals.i2s0,
            &config::StdConfig::philips(SAMPLE_RATE, config::DataBitWidth::Bits32),
            pins.gpio12,              // bclk
            pins.gpio13,              // d-out
            Option::<AnyIOPin>::None, // mclk
            pins.gpio14,              // ws (l-r-clk)
        )?;
    i2s_out.tx_enable()?;

    // let wave_table_size = 64;
    //
    // let synth = Arc::new(Mutex::new(Synth::new(wave_table_size, SAMPLE_RATE)));
    //
    // let mut ctrl = Scanner {
    //     pressed: BTreeMap::new(),
    //     octave: 3,
    //     tick_i: 0,
    //     trem_conf: GenEffectConf::new(),
    //     echo_conf: GenEffectConf::new(),
    //     octave_up: false,
    //     octave_down: false,
    //     columns: [
    //         PinDriver::output(<Gpio42 as Into<AnyOutputPin>>::into(pins.gpio42))?,
    //         PinDriver::output(<Gpio20 as Into<AnyOutputPin>>::into(pins.gpio20))?,
    //         PinDriver::output(<Gpio48 as Into<AnyOutputPin>>::into(pins.gpio48))?,
    //         PinDriver::output(<Gpio45 as Into<AnyOutputPin>>::into(pins.gpio45))?,
    //         PinDriver::output(<Gpio35 as Into<AnyOutputPin>>::into(pins.gpio35))?,
    //         PinDriver::output(<Gpio36 as Into<AnyOutputPin>>::into(pins.gpio36))?,
    //         PinDriver::output(<Gpio38 as Into<AnyOutputPin>>::into(pins.gpio38))?,
    //         PinDriver::output(<Gpio39 as Into<AnyOutputPin>>::into(pins.gpio39))?,
    //     ],
    //     rows: [
    //         PinDriver::input(<Gpio41 as Into<AnyIOPin>>::into(pins.gpio41))?,
    //         PinDriver::input(<Gpio47 as Into<AnyIOPin>>::into(pins.gpio47))?,
    //         PinDriver::input(<Gpio40 as Into<AnyIOPin>>::into(pins.gpio40))?,
    //         PinDriver::input(<Gpio37 as Into<AnyIOPin>>::into(pins.gpio37))?,
    //     ],
    //     adc: ADC {
    //         pitch: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio4)?,
    //         vol: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio5)?,
    //         attack: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio7)?,
    //         decay: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio6)?,
    //         trem_vol: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio16)?,
    //         trem_speed: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio15)?,
    //         echo_vol: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio18)?,
    //         echo_speed: adc::AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pins.gpio17)?,
    //         driver1: adc::AdcDriver::new(
    //             peripherals.adc1,
    //             &adc::config::Config::new().calibration(true),
    //         )?,
    //         driver2: adc::AdcDriver::new(
    //             peripherals.adc2,
    //             &adc::config::Config::new().calibration(true),
    //         )?,
    //     },
    //     synth: synth.clone(),
    //     vol: 1.0,
    //     pitch: 0,
    //     attack: 0.0,
    //     decay: 0.0,
    //     buttons: Buttons {
    //         octave_up: PinDriver::input(<Gpio9 as Into<AnyInputPin>>::into(pins.gpio9))?,
    //         octave_down: PinDriver::input(<Gpio10 as Into<AnyInputPin>>::into(pins.gpio10))?,
    //         tremolo: PinDriver::input(<Gpio11 as Into<AnyInputPin>>::into(pins.gpio11))?,
    //         echo: PinDriver::input(<Gpio21 as Into<AnyInputPin>>::into(pins.gpio21))?,
    //     },
    // };

    // ctrl.init()?;
    let mut ctrl = uart::UartCtrlr::new(peripherals.uart1, (pins.gpio0, pins.gpio1), peripherals.uart2, (pins.gpio5, pins.gpio4))?;

    info!("cloning synth struct...");
    let synth = ctrl.synth.clone();
    info!("cloning synth struct.");

    ThreadSpawnConfiguration {
        name: Some("audio-playback\0".as_bytes()),
        pin_to_core: Some(Core::Core1),
        ..Default::default()
    }
    .set()?;

    thread::spawn(move || 
        loop {
            let sample = synth.lock().unwrap().get_sample();

            // info!("{sample:?}");
            if let Err(why) = i2s_out.write(&[sample[0], sample[1], sample[2], sample[3], sample[0], sample[1], sample[2], sample[3]], BLOCK) {
            // if let Err(why) = i2s_out.write(&[sample.0, sample.1, sample.0, sample.1], BLOCK) {
                error!("could not send data bc {why}");
            }
        }
    );

    // ThreadSpawnConfiguration {
    //     name: Some("\0".as_bytes()),
    //     pin_to_core: Some(Core::Core0),
    //     ..Default::default()
    // }
    // .set()?;

    // let _ = thread::spawn(move || {
    loop {
        if let Err(e) = ctrl.step() {
            error!("controller step failed with error: {e}");
        }

        if let Err(e) = ctrl.step_2() {
            error!("controller step_2 failed with error: {e}");
        }
    }
    // })
    // .join();

    info!("*** NOW EXITING ***");
    Ok(())
}
