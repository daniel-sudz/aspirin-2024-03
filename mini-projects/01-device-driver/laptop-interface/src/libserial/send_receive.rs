use anyhow::Result;
use std::ffi::CString;
use std::os::raw::c_void;

use crate::libserial::ffi::{
    sp_blocking_read, sp_blocking_write, sp_drain, sp_open, sp_set_baudrate, sp_set_bits,
    sp_set_flowcontrol, sp_set_parity, sp_set_stopbits, SpFLOWCONTROL, SpMODE, SpPARITY,
};

use super::ffi::SpReturn;
use super::types::Port;

/// Checks the result of a serial port operation and returns `Ok(())` if successful.
/// If the operation fails, it returns an error with a description of the failure.
pub fn check(result: SpReturn) -> Result<()> {
    match result {
        SpReturn::SpOK => Ok(()),
        _ => Err(anyhow::anyhow!("Error: {:?}", result)),
    }
}

/// Configures the serial port for communication by setting parameters such as
/// baud rate, data bits, parity, stop bits, and flow control.
///
/// # Arguments
/// * `port` - Reference to the serial port to configure.
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

/// Sends data to the serial port.
///
/// # Arguments
/// * `port` - Reference to the serial port to send data to.
/// * `data` - The string data to be sent.
///
/// Converts the string data to a C-compatible string, writes it to the serial port,
/// and ensures all data is flushed to the port.
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

/// Receives data from the serial port.
///
/// Reads bytes from the serial port into a buffer until a newline character
/// is encountered or no more data is available.
///
/// # Arguments
/// * `port` - Reference to the serial port to receive data from.
///
/// # Returns
/// A `String` containing the received data.
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
                // No data received, break the loop.
                break;
            }
            _ => {
                // Check for newline character (indicating end of message).
                match buffer[buffer_idx] {
                    10 => {
                        break;
                    }
                    _ => {
                        // Continue reading, increment buffer index.
                        buffer_idx += 1;
                    }
                }
            }
        }
    }
    // Convert the received bytes into a string, trimming any trailing whitespace.
    let result = String::from_utf8_lossy(&buffer[..buffer_idx])
        .trim()
        .to_string();
    Ok(result)
}
