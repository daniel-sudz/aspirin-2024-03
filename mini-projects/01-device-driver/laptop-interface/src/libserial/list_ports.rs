use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_int};
use anyhow::Result;

// Define the sp_port struct as an opaque type
#[repr(C)]
pub struct sp_port {
    _private: [u8; 0],
}


#[repr(C)]
#[derive(Debug, PartialEq)]
// https://sigrok.org/api/libserialport/unstable/a00017.html#a8fa8ba0dd105754372ca82a6f391091e
pub enum SpReturn {
    SP_OK = 0,
    SP_ERR_ARG = -1,
    SP_ERR_FAIL = -2,
    SP_ERR_SUPP = -3,
}


// FFI bindings to libserialport functions
extern "C" {
    // int sp_list_ports(struct sp_port ***port_list);
    fn sp_list_ports(port_list: *mut *mut *mut sp_port) -> SpReturn;

    // const char *sp_get_port_name(const struct sp_port *port);
    fn sp_get_port_name(port: *const sp_port) -> *const c_char;

    // void sp_free_port_list(struct sp_port **port_list);
    fn sp_free_port_list(port_list: *mut *mut sp_port);
}

// Example output: 
// ---- list_ports::tests::test_list_ports stdout ----
// Port name: /dev/cu.Bluetooth-Incoming-Port
// Port name: /dev/cu.usbmodem2101
pub fn list_ports() -> Vec<String> {
    unsafe {
        let mut port_list: *mut *mut sp_port = ptr::null_mut();
        let result: SpReturn = sp_list_ports(&mut port_list);

        let mut ports: Vec<String> = Vec::new();

        match result {
            SpReturn::SP_OK => {
                let mut i = 0;
                while !(*port_list.add(i)).is_null() { 
                    let port = *port_list.add(i);
                    let port_name = sp_get_port_name(port);
                    if !port_name.is_null() {
                        let port_name_str = CStr::from_ptr(port_name).to_string_lossy();
                        ports.push(port_name_str.to_string());
                        println!("Port name: {}", port_name_str);
                    }
                    i += 1;
                }

                // Don't forget to free the port list when done
                sp_free_port_list(port_list);
            }
            _ => {
                println!("Error: {:?}", result);
            }
        }
        ports
    }
}

pub fn get_rpi_port() -> Result<String> {
    let ports = list_ports();
    let result = ports.iter().find(|port| port.contains("cu.usbmodem"));
    if let Some(port) = result {
        Ok(port.to_string())
    } else {
        Err(anyhow::anyhow!("No RPi port found"))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_ports() {
        list_ports();
    }
}
