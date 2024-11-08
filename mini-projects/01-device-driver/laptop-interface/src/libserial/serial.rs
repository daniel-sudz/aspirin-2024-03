use anyhow::Result;

use super::{list_ports::get_rpi_port, send_receive::{configure_send_receive, send_receive}};
use super::types::Port;

/// A safe-rust wrapper for interacting with libserialport using C FFI bindings
pub struct Serial {
    pub port: Port,
}

impl Serial {

    /// Ceates a new Serial instance by automatically finding the RPi port
    pub fn from_auto_configure() -> Result<Self> {
        let port = get_rpi_port()?;
        configure_send_receive(&port)?;
        Ok(Self { port })
    }

    /// blocking sends a message to the serial port
    pub fn send(&self, message: String) -> Result<()> {
        send_receive(&self.port, message)
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
}
