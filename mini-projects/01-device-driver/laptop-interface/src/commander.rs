use crate::libserial::serial::Serial;
use anyhow::Result;
use rayon::prelude::*;
use std::thread;

use crate::libserial::types::Port;
use crate::threads::BufferedBackgroundSerial;

pub struct Commander {
    serial: BufferedBackgroundSerial,
    pos: (i32, i32),
}

unsafe impl Send for Commander {}

impl Commander {
    pub fn from_auto_configure() -> Result<Self> {
        let serial = Serial::from_auto_configure()?;
        Ok(Self {
            serial: BufferedBackgroundSerial::from_serial(serial),
            pos: (0, 0),
        })
    }
    pub fn from_port(port: Port) -> Result<Self> {
        let serial = Serial::from_port(port)?;
        Ok(Self {
            serial: BufferedBackgroundSerial::from_serial(serial),
            pos: (0, 0),
        })
    }

    pub fn get_pos(&self) -> (i32, i32) {
        self.serial.get_pos()
    }

    pub fn run_on_all_commanders(
        commanders: &mut Vec<Commander>,
        f: impl Fn(&mut Commander) -> Result<()> + Send + std::marker::Copy,
    ) -> Result<()> {
        thread::scope(move |s| {
            for commander in commanders {
                s.spawn(move || f(commander));
            }
        });
        Ok(())
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
        self.serial.send("start controller".to_string())
    }

    /// Transitions from DeviceState::Running to DeviceState::Complete
    pub fn transition_to_complete(&self) -> Result<()> {
        self.serial.send("stop controller".to_string())
    }

    /// Transitions from DeviceState::Complete to DeviceState::PendingInit
    pub fn transition_to_pending_init_from_complete(&self) -> Result<()> {
        self.serial.send("reset".to_string())
    }

    /// Transitions from DeviceState::Complete to DeviceState::PendingStart
    pub fn transition_to_pending_start_from_complete(&self) -> Result<()> {
        self.serial.send("restart".to_string())
    }

    /// Transitions from DeviceState::Complete to DeviceState::Running
    pub fn transition_to_running_from_complete(&self) -> Result<()> {
        self.serial.send("start controller".to_string())
    }

    /// Turns on debug mode
    pub fn set_debug_mode_on(&self) -> Result<()> {
        self.serial.send("enable debug".to_string())
    }

    /// Turns off debug mode
    pub fn set_debug_mode_off(&self) -> Result<()> {
        self.serial.send("disable debug".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commander() {
        let mut commander = Commander::from_auto_configure().unwrap();
        commander.set_debug_mode_off().unwrap();
        commander.transition_to_pending_start().unwrap();
        commander.set_ready_led().unwrap();
        commander.set_all_leds().unwrap();

        commander.transition_to_running().unwrap();

        for _ in 0..20 {
            //println!("Checking for update");
            let pos = commander.get_pos();
            println!("Pos: {:?}", pos);
            thread::sleep(std::time::Duration::from_millis(100));
        }

        commander.transition_to_complete().unwrap();
        commander
            .transition_to_pending_init_from_complete()
            .unwrap();
        //commander.set_go_led().unwrap();
    }
}
