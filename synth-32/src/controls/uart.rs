use super::scanner::Scanner;
use crate::Float;
use anyhow::Result;
use esp_idf_svc::hal::{
    delay::{BLOCK, NON_BLOCK},
    uart::{
        UartDriver,
        config::Config,
        Uart,
    },
    adc,
    gpio::*,
    peripheral::Peripheral,
};
use log::*;
use synth::{synth::Synth, SAMPLE_RATE};
use std::sync::{Arc, Mutex};
use uart_commands::{UartCommand, Entity};

pub mod uart_commands;

pub struct UartCtrlr<'a> {
    com: UartDriver<'a>,
    pub synth: Arc<Mutex<Synth>>,
}

impl<'a> UartCtrlr<'a> {
    pub fn new<UART: Uart>(uart: impl Peripheral<P = UART> + 'a, pins: (impl Peripheral<P = impl OutputPin> + 'a, impl Peripheral<P = impl InputPin> + 'a)) -> Result<Self> {
        let wave_table_size = 64;
        let synth = Arc::new(Mutex::new(Synth::new(wave_table_size, SAMPLE_RATE)));

        // make command UART
        let mut com = UartDriver::new(
            uart,
            pins.0, // tx
            pins.1, // rx
            Option::<AnyIOPin>::None,
            Option::<AnyIOPin>::None,
            &Config::new().baudrate(115200.into()),
        )?;

        Ok(Self {
            com,
            synth,
        })      
    }

    pub fn read_command(&mut self) -> Result<()> {
        let mut cmd_buf = [0; 5_000];

        if Ok(0) == self.com.read(&mut cmd_buf, NON_BLOCK) {
            return Ok(());
        }

        // let cmd = String::from_utf8_lossy(&cmd_buf).to_string();
        let cmd: UartCommand = serde_cbor::from_slice(&cmd_buf)?;

        // match cmd.as_str() {
        //     "sv" => {
        //         // let mut float_buff = [0; 4];
        //         // com.read(&mut float_buff, BLOCK)?;
        //         //
        //         // let vol = f32::from_ne_bytes(float_buff) as Float;
        //         //
        //         // if vol != ctrl.vol {
        //         //     self.synth.vol = vol;
        //         // }
        //     }
        //     _ => return Ok(()),
        // }

        Ok(())
    }

}

/// returns true if there are peripherals connected to the uart port.
pub fn peripherals_connected(com: &mut UartDriver) -> bool {
    // TODO: send ping on uart port to see if any nperipherals reply.

    false
}
