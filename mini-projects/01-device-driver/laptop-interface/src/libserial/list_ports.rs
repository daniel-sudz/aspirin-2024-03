use std::ffi::{CStr, CString};
use std::ptr;
use std::os::raw::{c_char, c_int};
use anyhow::Result;
use super::{types::Port, ffi::{SpReturn, sp_port, sp_list_ports, sp_get_port_name, sp_free_port_list}};

// Example output: 
// ---- list_ports::tests::test_list_ports stdout ----
// Port name: /dev/cu.Bluetooth-Incoming-Port
// Port name: /dev/cu.usbmodem2101
pub fn list_ports() -> Vec<Port> {
    unsafe {
        let mut port_list: *mut *mut sp_port = ptr::null_mut();
        let result: SpReturn = sp_list_ports(&mut port_list);

        let mut ports: Vec<Port> = Vec::new();

        match result {
            SpReturn::SP_OK => {
                let mut i = 0;
                while !(*port_list.add(i)).is_null() { 
                    let port = *port_list.add(i);
                    let port_name = sp_get_port_name(port);
                    if !port_name.is_null() {
                        let port_name_str = CStr::from_ptr(port_name).to_string_lossy();
                        ports.push(Port {
                            name: port_name_str.to_string(),
                            handle: port,
                        });
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

/// Finds the RPi port by name matching the first device containing "cu.usbmodem"
pub fn get_rpi_port() -> Result<Port> {
    let ports = list_ports();
    let result = ports.into_iter().find(|port| port.name.contains("cu.usbmodem"));
    if let Some(port) = result {
        Ok(port)
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