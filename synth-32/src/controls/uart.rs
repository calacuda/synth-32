use super::scanner::Scanner;
use crate::Float;
use anyhow::{Result, bail};
use esp_idf_svc::hal::{
    delay::{BLOCK, NON_BLOCK},
    uart::{
        UartDriver,
        config::Config,
        Uart,
        UART2,
    },
    adc,
    gpio::*,
    peripheral::Peripheral,
};
use log::*;
use synth::{synth::Synth, SAMPLE_RATE, notes::{NOTE_NAMES, NOTES}};
use std::sync::{Arc, Mutex};
use uart_commands::{UartCommand, Entity};

pub mod uart_commands;

pub struct UartCtrlr<'a> {
    com: UartDriver<'a>,
    com_2: UartDriver<'a>,
    pub synth: Arc<Mutex<Synth>>,
}

impl<'a> UartCtrlr<'a> {
    pub fn new<UART: Uart>(
        uart: impl Peripheral<P = UART> + 'a,
        pins: (impl Peripheral<P = impl OutputPin> + 'a, impl Peripheral<P = impl InputPin> + 'a),
        uart_2: impl Peripheral<P = UART2> + 'a,
        pins_2: (impl Peripheral<P = impl OutputPin> + 'a, impl Peripheral<P = impl InputPin> + 'a)
    ) -> Result<Self> {
        let wave_table_size = 64;
        let synth = Arc::new(Mutex::new(Synth::new(wave_table_size, SAMPLE_RATE)));

        info!("making UartDriver 1");
        // make command UART
        let mut com = UartDriver::new(
            uart,
            pins.0, // tx
            pins.1, // rx
            Option::<AnyIOPin>::None,
            Option::<AnyIOPin>::None,
            &Config::new().baudrate(460800.into()),
        )?;
        info!("UartDriver 1 made");

        info!("making UartDriver 2");
        // make command UART
        let mut com_2 = UartDriver::new(
            uart_2,
            pins_2.0, // tx
            pins_2.1, // rx
            Option::<AnyIOPin>::None,
            Option::<AnyIOPin>::None,
            &Config::new().baudrate(460800.into()),
        )?;
        info!("UartDriver 2 made");


        Ok(Self {
            com,
            com_2,
            synth,
        })      
    }

    fn interpret_cmd(&mut self, cmd: UartCommand) -> Result<()> {
        match (cmd.entity.clone(), cmd.set) {
            (Entity::Volume, true)=> {
                info!("setting volume to {}", cmd.args.unwrap());
                self.synth.lock().unwrap().volume = cmd.args.unwrap();
                // info!("not setting volume bc volume pot is broken!");
            }
            (Entity::Volume, false)=> {
                if let Err(e) = self.com.write(&self.synth.lock().unwrap().volume.to_le_bytes()) {
                    bail!("failed to send volume data. error: {e}");    
                }
            },
            (Entity::PlayNote, true) => {
                let note_name = NOTE_NAMES[cmd.args.unwrap() as usize];
                info!("playing note {note_name}");
                self.synth.lock().unwrap().play(*NOTES.get(note_name).unwrap())
            }
            (Entity::PlayNote, false) => {}
            (Entity::StopNote, true) => {
                // self.synth.lock().unwrap().stop(*NOTES.get(NOTE_NAMES[cmd.args.unwrap() as usize]).unwrap())
                let note_name = NOTE_NAMES[cmd.args.unwrap() as usize];
                info!("stopping note {note_name}");
                self.synth.lock().unwrap().stop(*NOTES.get(note_name).unwrap())
            }
            (Entity::StopNote, false) => {}
            (Entity::LowPassCutoff, true) => {self.synth.lock().unwrap().set_cutoff(cmd.args.unwrap())}
            (Entity::LowPassCutoff, false) => {}
            (Entity::LowPassResonance, true) => {
                self.synth.lock().unwrap().set_resonance(cmd.args.unwrap());
                // info!("not setting resonance bc the pot for that control is broken!");
            }
            (Entity::LowPassResonance, false) => {}

            (Entity::EnvAttack, true) => {self.synth.lock().unwrap().set_attack(cmd.args.unwrap())}
            (Entity::EnvAttack, false) => {}
            (Entity::EnvDecay, true) => {self.synth.lock().unwrap().set_decay(cmd.args.unwrap())}
            (Entity::EnvDecay, false) => {}
            // (Entity::EnvSustain, true) => {self.synth.lock().unwrap().set_sustain(cmd.args.unwrap())}
            (Entity::EnvSustain, false) => {}


            // (Entity::, true) => {}
            // (Entity::, false) => {}
            _ => {
                warn!("command {:?}-{} is not yet programmed", cmd.entity, if cmd.set { "set" } else { "get" });
            },
        }

        Ok(())
    }

    pub fn step(&mut self) -> Result<()> {
        // info!("step point 1");
        let mut cmd_buf = [0; 2_500];
        // info!("step point 2");

        if self.com.remaining_read().unwrap() > 0 {
            if let Ok(n) = self.com.read(&mut cmd_buf, NON_BLOCK) {
                if n == 0 {
                    return Ok(());
                }
                // let cmd = String::from_utf8_lossy(&cmd_buf).to_string();
                let cmd: UartCommand = serde_json::from_slice(&cmd_buf[..n])?;

                self.interpret_cmd(cmd)?;
            }
        }

        Ok(())
    }

    pub fn step_2(&mut self) -> Result<()> {
        // info!("step point 1");
        let mut cmd_buf = [0; 2_500];
        // info!(step point 2");

        if self.com_2.remaining_read().unwrap() > 0 {
            if let Ok(n) = self.com_2.read(&mut cmd_buf, NON_BLOCK) {
                if n == 0 {
                    return Ok(());
                }
                // let cmd = String::from_utf8_lossy(&cmd_buf).to_string();
                let cmd: Result<Vec<UartCommand>, _>= serde_json::from_slice(&cmd_buf[..n]);

                // warn!("{:?}", cmd);
                // warn!("{:?}", String::from_utf8_lossy(&cmd_buf[..n]));

                cmd?.into_iter().map(|cmd| {self.interpret_cmd(cmd)}).collect::<Result<()>>()?;
            }
        }

        Ok(())
    }

}

/// returns true if there are peripherals connected to the uart port.
pub fn peripherals_connected(com: &mut UartDriver) -> bool {
    // TODO: send ping on uart port to see if any nperipherals reply.

    false
}
