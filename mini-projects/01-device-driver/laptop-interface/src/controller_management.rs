use crate::commander::Commander;
use crate::libserial::list_ports::get_all_rpi_ports;
use crate::libserial::types::Port;

use super::libserial::serial::Serial;
use anyhow::Result;
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
pub enum DeviceState {
    PendingInit = 0,
    PendingStart = 1,
    Running = 2,
    Complete = 3,
}

#[derive(PartialEq)]
pub enum ControllerInput {
    Reset,
    Restart,
    StartController,
}

pub struct Device {
    state: DeviceState,
    commander: Commander,
}

pub struct MultiDevice {
    devices: Vec<Device>,
}

impl Device {
    pub fn from_auto_configure() -> Result<Self> {
        let commander = Commander::from_auto_configure()?;
        Ok(Self {
            state: DeviceState::PendingInit,
            commander,
        })
    }
    pub fn from_port(port: Port) -> Result<Self> {
        let commander = Commander::from_port(port)?;
        Ok(Self {
            state: DeviceState::PendingInit,
            commander,
        })
    }

    pub fn controller_state_change(
        &mut self,
        input: &Option<ControllerInput>,
    ) -> Result<()> {
        self.state = match self.state {
            DeviceState::PendingInit => {
                self.commander.transition_to_pending_start()?;
                DeviceState::PendingStart
            }
            DeviceState::PendingStart => {
                self.commander.set_ready_led()?;
                self.commander.set_set_led()?;
                thread::sleep(Duration::from_millis(1000));
                self.commander.set_go_led()?;
                thread::sleep(Duration::from_millis(1000));
                self.commander.set_all_leds_off()?;
                DeviceState::Running
            }
            DeviceState::Running => {
                self.commander.transition_to_complete()?;
                DeviceState::Complete
            }
            DeviceState::Complete => {
                if let Some(input_value) = input {
                    match input_value {
                        ControllerInput::Reset => {
                            self.commander.transition_to_pending_init_from_complete()?;
                            DeviceState::PendingInit
                        }
                        ControllerInput::Restart => {
                            self.commander.transition_to_pending_start_from_complete()?;
                            DeviceState::PendingStart
                        }
                        ControllerInput::StartController => {
                            self.commander.transition_to_running_from_complete()?;
                            DeviceState::Running
                        }
                    }
                } else {
                    eprintln!("No input provided for DeviceState::Complete");
                    DeviceState::Complete
                }
            }
        };
        Ok(())
    }
}

impl MultiDevice {
    pub fn from_auto_configure(num_devices: usize) -> Result<Self> {
        let ports = get_all_rpi_ports();
        let devices = match ports.len() == num_devices {
            true => ports.into_iter().map(|port| Device::from_port(port)).collect(),
            false => Err(anyhow::anyhow!("Not enough devices found"))
        }?;
        Ok(Self { devices })
    }

    pub fn controller_state_change(&mut self, input: &Option<ControllerInput>) -> Result<()> {
        for device in self.devices.iter_mut() {
            device.controller_state_change(input)?;
        }
        Ok(())
    }

    pub fn get_pos(&mut self) -> Vec<(i32, i32)> {
        self.devices.iter_mut().map(|device| device.commander.get_pos()).collect()
    }

}