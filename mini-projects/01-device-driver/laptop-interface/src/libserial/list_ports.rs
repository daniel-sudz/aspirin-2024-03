use super::{
    ffi::{sp_get_port_name, sp_list_ports, sp_port, SpReturn},
    types::Port,
};
use anyhow::Result;
use std::ffi::CStr;
use std::ptr;

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
    let result = ports
        .into_iter()
        .find(|port| port.name.contains("cu.usbmodem"));
    if let Some(port) = result {
        println!("RPi port selected: {}", port.name);
        Ok(port)
    } else {
        Err(anyhow::anyhow!("No RPi port found"))
    }
}

pub fn get_all_rpi_ports() -> Vec<Port> {
    let ports = list_ports();
    ports
        .into_iter()
        .filter(|port| port.name.contains("cu.usbmodem"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_ports() {
        list_ports();
    }
}
