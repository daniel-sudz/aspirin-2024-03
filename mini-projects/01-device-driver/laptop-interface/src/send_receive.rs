use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};

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

pub fn check(result: SpReturn) -> i32 {
    match result {
        SpReturn::SP_ERR_ARG => {
            println!("Error: Invalid argument.");
            std::process::abort();
        }
        SpReturn::SP_ERR_FAIL => unsafe {
            let error_message = sp_last_error_message();
            println!("Error: Failed: {}", CStr::from_ptr(error_message).to_string_lossy());
            sp_free_error_message(error_message);
            std::process::abort();
        },
        SpReturn::SP_ERR_SUPP => {
            println!("Error: Not supported.");
            std::process::abort();
        }
        SpReturn::SP_ERR_MEM => {
            println!("Error: Couldn't allocate memory.");
            std::process::abort();
        }
        SpReturn::SP_OK => 0,
    }
}

pub fn send_receive(port_names: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let num_ports = port_names.len();
    if num_ports < 1 || num_ports > 2 {
        return Err("Usage: Need 1 or 2 ports".into());
    }

    let mut ports: Vec<*mut sp_port> = vec![ptr::null_mut(); num_ports];

    unsafe {
        // Open and configure each port
        for i in 0..num_ports {
            println!("Looking for port {}.", port_names[i]);
            let port_name = CString::new(port_names[i].as_str())?;
            check(sp_get_port_by_name(port_name.as_ptr(), &mut ports[i]));
            
            println!("Opening port.");
            check(sp_open(ports[i], SpMode::SP_MODE_READ_WRITE));
            
            println!("Setting port to 9600 8N1, no flow control.");
            check(sp_set_baudrate(ports[i], 9600));
            check(sp_set_bits(ports[i], 8));
            check(sp_set_parity(ports[i], SpParity::SP_PARITY_NONE));
            check(sp_set_stopbits(ports[i], 1));
            check(sp_set_flowcontrol(ports[i], SpFlowControl::SP_FLOWCONTROL_NONE));
        }

        // Send and receive data
        for tx in 0..num_ports {
            let rx = if num_ports == 1 { 0 } else { if tx == 0 { 1 } else { 0 } };
            let tx_port = ports[tx];
            let rx_port = ports[rx];

            let data = "Hello!";
            let size = data.len();
            let timeout = 1000;

            println!("Sending '{}' ({} bytes) on port {}.", 
                data, size, 
                CStr::from_ptr(sp_get_port_name(tx_port)).to_string_lossy());

            let result = check(sp_blocking_write(tx_port, data.as_ptr() as *const c_void, size, timeout));
            
            if result == size as i32 {
                println!("Sent {} bytes successfully.", size);
            } else {
                println!("Timed out, {}/{} bytes sent.", result, size);
            }

            let mut buf = vec![0u8; size + 1];
            
            println!("Receiving {} bytes on port {}.", 
                size,
                CStr::from_ptr(sp_get_port_name(rx_port)).to_string_lossy());

            let result = check(sp_blocking_read(rx_port, buf.as_mut_ptr() as *mut c_void, size, timeout));

            if result == size as i32 {
                println!("Received {} bytes successfully.", size);
            } else {
                println!("Timed out, {}/{} bytes received.", result, size);
            }

            buf[result as usize] = 0;
            println!("Received '{}'.", std::str::from_utf8(&buf[..result as usize])?);
        }

        // Cleanup
        for i in 0..num_ports {
            check(sp_close(ports[i]));
            sp_free_port(ports[i]);
        }
    }

    Ok(())
}
