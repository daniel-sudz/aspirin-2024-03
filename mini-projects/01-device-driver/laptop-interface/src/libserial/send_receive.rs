use anyhow::Result;
use std::ffi::CString;
use std::os::raw::c_void;

use crate::libserial::ffi::{
    sp_blocking_read, sp_blocking_write, sp_drain, sp_open, sp_set_baudrate, sp_set_bits,
    sp_set_flowcontrol, sp_set_parity, sp_set_stopbits, SpFLOWCONTROL, SpMODE, SpPARITY,
};

use super::ffi::SpReturn;
use super::types::Port;

pub fn check(result: SpReturn) -> Result<()> {
    match result {
        SpReturn::SpOK => Ok(()),
        _ => Err(anyhow::anyhow!("Error: {:?}", result)),
    }
}

pub fn configure_send_receive(port: &Port) -> Result<()> {
    unsafe {
        check(sp_open(port.handle, SpMODE::SpModeReadWrite))?;
        check(sp_set_baudrate(port.handle, 115200))?;
        check(sp_set_bits(port.handle, 8))?;
        check(sp_set_parity(port.handle, SpPARITY::SpParityNONE))?;
        check(sp_set_stopbits(port.handle, 1))?;
        check(sp_set_flowcontrol(
            port.handle,
            SpFLOWCONTROL::SpFlowControlNONE,
        ))?;
        println!("Arduino port configured");
    }
    Ok(())
}

pub fn send(port: &Port, data: String) -> Result<()> {
    let c_data = CString::new(data.as_str())?;
    let data_ptr = c_data.as_ptr() as *const c_void;
    let data_len = data.len();

    let bytes_written = unsafe { sp_blocking_write(port.handle, data_ptr, data_len, 10000) as i32 };

    if bytes_written < 0 {
        Err(anyhow::anyhow!("Error sending data: {}", bytes_written))
    } else {
        unsafe { check(sp_drain(port.handle)) }?;
        println!("Sent {} bytes", bytes_written);
        Ok(())
    }
}

pub fn receive(port: &Port) -> Result<String> {
    let mut buffer = [0u8; 1024];
    let mut buffer_idx = 0;

    loop {
        let bytes_read = unsafe {
            sp_blocking_read(
                port.handle,
                buffer.as_mut_ptr().add(buffer_idx) as *mut c_void,
                1,
                5,
            ) as i32
        };
        match bytes_read {
            0 => {
                break;
            }
            _ => {
                // check for newline break
                match buffer[buffer_idx] {
                    10 => {
                        break;
                    }
                    _ => {
                        buffer_idx += 1;
                    }
                }
            }
        }
    }
    let result = String::from_utf8_lossy(&buffer[..buffer_idx])
        .trim()
        .to_string();
    //println!("total read: {} result: {}", buffer_idx, result);
    Ok(result)
}
