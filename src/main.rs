use anyhow::{bail, Result};
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::delay::BLOCK;
use esp_idf_svc::hal::gpio::AnyIOPin;
use esp_idf_svc::hal::i2s::{config, I2sDriver};
use esp_idf_svc::hal::peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use log::*;
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;

const TWOPI: f32 = (PI * 2.0) as f32;
const FREQ: f32 = 440.0;
const SAMPLE_RATE: u32 = 44_100;
const SCALE: [f32; 8] = [
    261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25,
];

#[derive(Clone)]
struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WavetableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> Self {
        Self {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}

fn convert(sample: f32) -> u8 {
    // (((sample + 1.0) / 2.0) * 255.0) as u8 // let sample = frame.channels()[0].to_f32();
    (((sample * 0.5) + 0.5) * 255.0) as u8 // let sample = frame.channels()[0].to_f32();
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

    let wave_table_size = 128;
    let wave_table: Vec<f32> = (0..wave_table_size)
        .map(|n| (TWOPI * n as f32 / wave_table_size as f32).sin())
        .collect();

    let oscillator = Arc::new(Mutex::new(WavetableOscillator::new(
        SAMPLE_RATE,
        wave_table.clone(),
    )));
    info!("generating buffer");
    // let mut buf = Vec::new();
    ThreadSpawnConfiguration {
        name: Some("audio-playback\0".as_bytes()),
        // stack_size: 980000,
        // priority: 15,
        pin_to_core: Some(Core::Core1),
        ..Default::default()
    }
    .set()?;

    let osc = oscillator.clone();
    // for _ in 0..SAMPLE_RATE {
    thread::spawn(move || loop {
        let sample = convert(oscillator.lock().unwrap().get_sample());
        if let Err(why) = i2s.write(&[sample, 0, sample, 0], BLOCK) {
            error!("could not send data bc {why}");
        }
    });

    ThreadSpawnConfiguration {
        name: Some("change-notes\0".as_bytes()),
        // stack_size: 98000,
        // priority: 15,
        pin_to_core: Some(Core::Core0),
        ..Default::default()
    }
    .set()?;

    let _ = thread::spawn(move || {
        loop {
            // synth.lock().unwrap().step();
            for note in SCALE {
                info!("adding note {note}");
                osc.lock().unwrap().set_frequency(note);
                // if let Err(why) = tx.send(0) {
                //     error!("could not send data bc {why}");
                // }
                // if let Err(why) = tx.send(Some(synth.buff.clone())) {
                //     error!("could not send data bc {why}");
                // }
                // if let Err(why) = tx.send(Some(synth.buff.clone())) {
                //     error!("could not send data bc {why}");
                // }
                FreeRtos::delay_us(250_000);

                osc.lock().unwrap().set_frequency(0.0);
                // synth.lock().unwrap().rm_note(note);
                // info!("notes: {:?}", synth.notes);
            }
            info!("*** done ***");
            // if let Err(why) = tx.send(Some(synth.buff.clone())) {
            //     error!("could not send data bc {why}");
            // }
            FreeRtos::delay_us(1_000_000);
        }
    })
    .join();

    Ok(())
}
