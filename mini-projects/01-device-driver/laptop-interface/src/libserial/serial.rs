use anyhow::Result;

use super::types::Port;
use super::{
    list_ports::get_rpi_port,
    send_receive::{configure_send_receive, receive, send},
};

/// A safe-rust wrapper for interacting with libserialport using C FFI bindings
pub struct Serial {
    pub port: Port,
}

unsafe impl Sync for Serial {}
unsafe impl Send for Serial {}

impl Serial {
    /// Ceates a new Serial instance by automatically finding the RPi port
    pub fn from_auto_configure() -> Result<Self> {
        let port = get_rpi_port()?;
        configure_send_receive(&port)?;
        Ok(Self { port })
    }
    /// Create a new Serial instance from a specific port
    pub fn from_port(port: Port) -> Result<Self> {
        configure_send_receive(&port)?;
        Ok(Self { port })
    }

    /// blocking sends a message to the serial port
    pub fn send(&self, message: String) -> Result<()> {
        send(&self.port, message)
    }

    pub fn receive(&self) -> Result<String> {
        receive(&self.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send() {
        let serial = Serial::from_auto_configure().unwrap();
        serial.send("Hello, world!".to_string()).unwrap();
    }

    #[test]
    fn test_receive() {
        let serial = Serial::from_auto_configure().unwrap();
        let _data = serial.receive().unwrap();
    }
}
