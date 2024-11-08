use anyhow::Result;
use crate::libserial::serial::Serial;

/// A commander struct that provides high-level commands for the laptop interface
struct Commander {
    serial: Serial,
}

impl Commander {
    pub fn new() -> Result<Self> {
        let serial = Serial::from_auto_configure()?;
        Ok(Self { serial })
    }

    /// Transitions from DeviceState::PendingInit to DeviceState::PendingStart
    pub fn transition_to_pending_start(&self) -> Result<()> {
        self.serial.send("init controller".to_string())
    }

    /// Sets ready LED while in DeviceState::PendingStart
    /// This turns on the red LED and turns off all other LEDs
    pub fn set_ready_led(&self) -> Result<()> {
        self.serial.send("set ready led".to_string())
    }

    /// Sets set LED while in DeviceState::PendingStart
    /// This turns on the yellow LED and turns off all other LEDs
    pub fn set_set_led(&self) -> Result<()> {
        self.serial.send("set set led".to_string())
    }

    /// Sets go LED while in DeviceState::PendingStart
    /// This turns on the green LED and turns off all other LEDs
    pub fn set_go_led(&self) -> Result<()> {
        self.serial.send("set go led".to_string())
    }

    /// Sets all LEDs on while in DeviceState::PendingStart
    /// This turns on all three LEDs
    pub fn set_all_leds(&self) -> Result<()> {
        self.serial.send("set all leds".to_string())
    }

    /// Sets all LEDs off while in DeviceState::PendingStart
    /// This turns off all three LEDs
    pub fn set_all_leds_off(&self) -> Result<()> {
        self.serial.send("clear all leds".to_string())
    }

    /// Transitions from DeviceState::PendingStart to DeviceState::Running
    pub fn transition_to_running(&self) -> Result<()> {
        self.serial.send("stop controller".to_string())
    }
}
