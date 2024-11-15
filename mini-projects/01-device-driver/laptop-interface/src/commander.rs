use std::thread;
use rayon::prelude::*;
use anyhow::Result;
use crate::libserial::serial::Serial;

use crate::threads::BufferedBackgroundSerial;

pub struct Commander {
    serial: BufferedBackgroundSerial,
    pos: (i32, i32),
}

impl Commander {
    pub fn new() -> Result<Self> {
        let serial = Serial::from_auto_configure()?;
        Ok(Self { serial: BufferedBackgroundSerial::from_serial(serial), pos: (0,0) })
    }

    /// Updates the controller with the latest state
    pub fn update(&mut self, message: String) {
        // todo
        //println!("Updating controller with message: {}", message);
    }

    /// Checks for an update from the controller
    pub fn check_update(&mut self) {
        match self.serial.receive() {
            Ok(data) => {
                self.update(data);
            }
            Err(_) => {}
        }
    }

    pub fn get_pos(&mut self) -> (i32, i32) {
        self.check_update();
        self.pos
    }


    /// Transitions from DeviceState::PendingInit to DeviceState::PendingStart
    pub fn transition_to_pending_start(&self) -> Result<()> {
        self.serial.send("init controller".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Sets ready LED while in DeviceState::PendingStart
    /// This turns on the red LED and turns off all other LEDs
    pub fn set_ready_led(&self) -> Result<()> {
        self.serial.send("set ready led".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Sets set LED while in DeviceState::PendingStart
    /// This turns on the yellow LED and turns off all other LEDs
    pub fn set_set_led(&self) -> Result<()> {
        self.serial.send("set set led".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Sets go LED while in DeviceState::PendingStart
    /// This turns on the green LED and turns off all other LEDs
    pub fn set_go_led(&self) -> Result<()> {
        self.serial.send("set go led".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Sets all LEDs on while in DeviceState::PendingStart
    /// This turns on all three LEDs
    pub fn set_all_leds(&self) -> Result<()> {
        self.serial.send("set all leds".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Sets all LEDs off while in DeviceState::PendingStart
    /// This turns off all three LEDs
    pub fn set_all_leds_off(&self) -> Result<()> {
        self.serial.send("clear all leds".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Transitions from DeviceState::PendingStart to DeviceState::Running
    pub fn transition_to_running(&self) -> Result<()> {
        self.serial.send("start controller".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Transitions from DeviceState::Running to DeviceState::Complete
    pub fn transition_to_complete(&self) -> Result<()> {
        self.serial.send("stop controller".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Transitions from DeviceState::Complete to DeviceState::PendingInit
    pub fn transition_to_pending_init_from_complete(&self) -> Result<()> {
        self.serial.send("reset".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Transitions from DeviceState::Complete to DeviceState::PendingStart
    pub fn transition_to_pending_start_from_complete(&self) -> Result<()> {
        self.serial.send("restart".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Transitions from DeviceState::Complete to DeviceState::Running
    pub fn transition_to_running_from_complete(&self) -> Result<()> {
        self.serial.send("start controller".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Turns on debug mode
    pub fn set_debug_mode_on(&self) -> Result<()> {
        self.serial.send("enable debug".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    /// Turns off debug mode
    pub fn set_debug_mode_off(&self) -> Result<()> {
        self.serial.send("disable debug".to_string())?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commander() {
        let mut commander = Commander::new().unwrap();
        commander.set_debug_mode_on().unwrap();
        commander.transition_to_pending_start().unwrap();
        commander.set_ready_led().unwrap();
        commander.set_all_leds().unwrap();

        commander.transition_to_running().unwrap();

        for _ in 0..100 {
            //println!("Checking for update");
            let pos = commander.get_pos();
            println!("Pos: {:?}", pos);
            thread::sleep(std::time::Duration::from_millis(100));
        }

        commander.transition_to_complete().unwrap();
        commander.transition_to_pending_init_from_complete().unwrap();
        //commander.set_go_led().unwrap();
    }
}
