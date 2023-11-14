use super::scanner::Scanner;
use crate::Float;
use anyhow::Result;
use esp_idf_svc::hal::{
    delay::{BLOCK, NON_BLOCK},
    uart::UartDriver,
};

pub fn read_command(com: &mut UartDriver, ctrl: &mut Scanner) -> Result<()> {
    let mut cmd_buf = [0; 2];

    if Ok(0) == com.read(&mut cmd_buf, NON_BLOCK) {
        return Ok(());
    }

    let cmd = String::from_utf8_lossy(&cmd_buf).to_string();

    match cmd.as_str() {
        "gv" => {
            let mut float_buff = [0; 8];
            com.read(&mut float_buff, BLOCK)?;

            let vol = f64::from_ne_bytes(float_buff) as Float;

            if vol != ctrl.vol {
                ctrl.vol = vol;
            }
        }
        _ => return Ok(()),
    }

    Ok(())
}

/// returns true if there are peripherals connected to the uart port.
pub fn peripherals_connected(com: &mut UartDriver) -> bool {
    // TODO: send ping on uart port to see if any nperipherals reply.

    false
}
