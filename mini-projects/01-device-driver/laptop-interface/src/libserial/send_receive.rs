use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use anyhow::Result;
use libc::strlen;

use crate::libserial::ffi::{sp_blocking_write, sp_open, sp_set_baudrate, sp_set_bits, sp_set_flowcontrol, sp_set_parity, sp_set_stopbits, SpFlowControl, SpMode, SpParity};

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
        let _ = check(sp_set_flowcontrol(port.handle, SpFlowControl::SP_FLOWCONTROL_NONE))?;
        println!("Arduino port configured");
    }
    Ok(())
}

pub fn send_receive(port: &Port, data: String) -> Result<()> {
    unsafe {
        let data = CString::new(data.as_str())?;
        let data_len: usize = strlen(data.as_ptr());
        println!("data_len: {}", data_len);
        let bytes_written: i32 = sp_blocking_write(port.handle, data.as_ptr() as *const c_void, data_len, 10000) as i32;
        println!("bytes_written: {}", bytes_written);
    }
    Ok(())
}


mod tests {
    use crate::libserial::list_ports::get_rpi_port;

    use super::*;


    #[test]
    fn test_send_receive() {
        let port = get_rpi_port().unwrap();
        println!("port: {}", port.name);
        configure_send_receive(&port).unwrap();
        send_receive(&port, "init controller".to_string()).unwrap();
        send_receive(&port, "set ready led".to_string()).unwrap();
    }
}
