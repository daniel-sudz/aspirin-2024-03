use anyhow::Result;
use libc::strlen;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

use crate::libserial::ffi::{
    sp_blocking_read, sp_blocking_read_next, sp_blocking_write, sp_drain, sp_open, sp_set_baudrate, sp_set_bits, sp_set_flowcontrol,
    sp_set_parity, sp_set_stopbits, SpFlowControl, SpMode, SpParity,
};

use super::ffi::{sp_port, SpReturn};
use super::types::Port;

pub fn check(result: SpReturn) -> Result<()> {
    match result {
        SpReturn::SP_OK => Ok(()),
        _ => Err(anyhow::anyhow!("Error: {:?}", result)),
    }
}

pub fn configure_send_receive(port: &Port) -> Result<()> {
    unsafe {
        let _ = check(sp_open(port.handle, SpMode::SP_MODE_READ_WRITE))?;
        let _ = check(sp_set_baudrate(port.handle, 115200))?;
        let _ = check(sp_set_bits(port.handle, 8))?;
        let _ = check(sp_set_parity(port.handle, SpParity::SP_PARITY_NONE))?;
        let _ = check(sp_set_stopbits(port.handle, 1))?;
        let _ = check(sp_set_flowcontrol(
            port.handle,
            SpFlowControl::SP_FLOWCONTROL_NONE,
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
        let _ = unsafe { check(sp_drain(port.handle)) }?;
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
                100,
            ) as i32
        };
        match bytes_read {
            0 => {
                break;
            }
            _ => {
                buffer_idx += 1;
                // check for newline break
                match buffer[buffer_idx - 1] {
                    10 => {
                        break;
                    }
                    _ => {}
                }
            }
        }
  }
  let result = String::from_utf8_lossy(&buffer[..buffer_idx]).trim().to_string();
  println!("total read: {} result: {}", buffer_idx, result);
  Ok(result)
}

mod tests {
    use crate::libserial::list_ports::get_rpi_port;

    use super::*;

    #[test]
    fn test_send() {
        let port = get_rpi_port().unwrap();
        println!("port: {}", port.name);
        configure_send_receive(&port).unwrap();
        // send(&port, "init controller".to_string()).unwrap();
        // send(&port, "set ready led".to_string()).unwrap();
        // send(&port, "clear all leds".to_string()).unwrap();
        send(&port, "start controller".to_string()).unwrap();
        // send(&port, "stop controller".to_string()).unwrap();
        // send(&port, "reset".to_string()).unwrap();
        // send(&port, "enable debug".to_string()).unwrap();
        // send(&port, "disable debug".to_string()).unwrap();
    }

    #[test]
    fn test_receive() {
        let port = get_rpi_port().unwrap();
        println!("port: {}", port.name);
        configure_send_receive(&port).unwrap();
        let data = receive(&port).unwrap();
        println!("data: {}", data);
    }
}
