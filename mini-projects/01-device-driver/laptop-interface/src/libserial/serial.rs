use anyhow::Result;

use super::{list_ports::{get_rpi_port}, send_receive::configure_send_receive};
use super::types::Port;
struct Serial {
    pub port: Port,
}

impl Serial {

    /// Ceates a new Serial instance by automatically finding the RPi port
    pub fn from_auto_configure() -> Result<Self> {
        let port = get_rpi_port()?;
        configure_send_receive(&port)?;
        Ok(Self { port })
    }
}