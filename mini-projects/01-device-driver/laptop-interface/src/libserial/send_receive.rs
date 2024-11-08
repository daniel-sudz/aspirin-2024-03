use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use anyhow::Result;
use libc::strlen;

#[repr(C)]
pub struct sp_port {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum SpReturn {
    SP_OK = 0,
    SP_ERR_ARG = -1,
    SP_ERR_FAIL = -2,
    SP_ERR_SUPP = -3,
    SP_ERR_MEM = -4,
}

#[repr(C)]
pub enum SpMode {
    SP_MODE_READ = 1,
    SP_MODE_WRITE = 2,
    SP_MODE_READ_WRITE = 3,
}

#[repr(C)]
pub enum SpParity {
    SP_PARITY_NONE = 0,
    SP_PARITY_ODD = 1,
    SP_PARITY_EVEN = 2,
}

#[repr(C)]
pub enum SpFlowControl {
    SP_FLOWCONTROL_NONE = 0,
    SP_FLOWCONTROL_XONXOFF = 1,
    SP_FLOWCONTROL_RTSCTS = 2,
    SP_FLOWCONTROL_DTRDSR = 3,
}

extern "C" {
    fn sp_get_port_by_name(portname: *const c_char, port_ptr: *mut *mut sp_port) -> SpReturn;
    fn sp_open(port: *mut sp_port, flags: SpMode) -> SpReturn;
    fn sp_close(port: *mut sp_port) -> SpReturn;
    fn sp_free_port(port: *mut sp_port);
    fn sp_get_port_name(port: *const sp_port) -> *const c_char;
    fn sp_set_baudrate(port: *mut sp_port, baudrate: c_int) -> SpReturn;
    fn sp_set_bits(port: *mut sp_port, bits: c_int) -> SpReturn;
    fn sp_set_parity(port: *mut sp_port, parity: SpParity) -> SpReturn;
    fn sp_set_stopbits(port: *mut sp_port, stopbits: c_int) -> SpReturn;
    fn sp_set_flowcontrol(port: *mut sp_port, flowcontrol: SpFlowControl) -> SpReturn;
    fn sp_blocking_write(port: *mut sp_port, buf: *const c_void, count: usize, timeout_ms: c_int) -> SpReturn;
    fn sp_blocking_read(port: *mut sp_port, buf: *mut c_void, count: usize, timeout_ms: c_int) -> SpReturn;
    fn sp_last_error_message() -> *mut c_char;
    fn sp_free_error_message(message: *mut c_char);
}

pub fn check(result: SpReturn) -> Result<()> {
    match result {
        SpReturn::SP_OK => Ok(()),
        _ => Err(anyhow::anyhow!("Error: {:?}", result)),
    }
}

pub fn configure_send_receive(port_name: String) -> Result<()> {
    let mut port: *mut sp_port = ptr::null_mut();

    unsafe {
        let port_name = CString::new(port_name.as_str())?;

        let _ = check(sp_get_port_by_name(port_name.as_ptr(), &mut port))?;
        let _ = check(sp_open(port, SpMode::SP_MODE_READ_WRITE))?;
        let _ = check(sp_set_baudrate(port, 115200))?;
        let _ = check(sp_set_bits(port, 8))?;
        let _ = check(sp_set_parity(port, SpParity::SP_PARITY_NONE))?;
        let _ = check(sp_set_stopbits(port, 1))?;
        let _ = check(sp_set_flowcontrol(port, SpFlowControl::SP_FLOWCONTROL_NONE))?;
        println!("Arduino port configured");
    }
    Ok(())
}

pub fn send_receive(port_name: String, data: String) -> Result<()> {
    let mut port: *mut sp_port = ptr::null_mut();
    unsafe {
        let port_name = CString::new(port_name.as_str())?;
        let _ = check(sp_get_port_by_name(port_name.as_ptr(), &mut port))?;
        let data = CString::new(data.as_str())?;
        let data_len: usize = strlen(data.as_ptr());

        println!("data_len: {}", data_len);

        let _ = check(sp_get_port_by_name(port_name.as_ptr(), &mut port))?;

        let bytes_written: i32 = sp_blocking_write(port, data.as_ptr() as *const c_void, data_len, 10000) as i32;
        println!("bytes_written: {}", bytes_written);
    }
    Ok(())
}


mod tests {
    use crate::list_ports::get_rpi_port;

    use super::*;

    #[test]
    fn test_configure_send_receive() {
        configure_send_receive(get_rpi_port().unwrap()).unwrap();
    }

    #[test]
    fn test_send_receive() {
        let port = get_rpi_port().unwrap();
        println!("port: {}", port);
        configure_send_receive(port.clone()).unwrap();
        send_receive(port.clone(), "init controller".to_string()).unwrap();
        send_receive(port.clone(), "set ready led".to_string()).unwrap();
    }
}
