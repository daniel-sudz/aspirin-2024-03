use crate::commander::Commander;

use super::libserial::serial::Serial;
use anyhow::Result;
use std::thread;
use std::time::Duration;

#[derive(PartialEq)]
enum DeviceState {
    PendingInit = 0,
    PendingStart = 1,
    Running = 2,
    Complete = 3,
}

#[derive(PartialEq)]
enum ControllerInput {
    Reset,
    Restart,
    StartController,
}

pub struct Device {
    state: DeviceState,
    commander: Commander,
}

impl Device {
    pub fn new() -> Result<Self> {
        let commander = Commander::new()?;
        Ok(Self {
            state: DeviceState::PendingInit,
            commander,
        })
    }

    pub fn controller_state_change(
        &mut self,
        input: Option<ControllerInput>,
    ) -> Result<DeviceState> {
        match self.state {
            DeviceState::PendingInit => {
                self.commander.transition_to_pending_start()?;
                Ok(DeviceState::PendingStart)
            }
            DeviceState::PendingStart => {
                self.commander.set_ready_led()?;
                thread::sleep(Duration::from_millis(1000));
                self.commander.set_set_led()?;
                thread::sleep(Duration::from_millis(1000));
                self.commander.set_go_led()?;
                thread::sleep(Duration::from_millis(1000));
                self.commander.set_all_leds_off()?;
                Ok(DeviceState::Running)
            }
            DeviceState::Running => {
                self.commander.transition_to_complete()?;
                Ok(DeviceState::Complete)
            }
            DeviceState::Complete => {
                if let Some(input_value) = input {
                    match input_value {
                        ControllerInput::Reset => {
                            self.commander.transition_to_pending_init_from_complete()?;
                            Ok(DeviceState::PendingInit)
                        }
                        ControllerInput::Restart => {
                            self.commander.transition_to_pending_start_from_complete()?;
                            Ok(DeviceState::PendingStart)
                        }
                        ControllerInput::StartController => {
                            self.commander.transition_to_running_from_complete()?;
                            Ok(DeviceState::Running)
                        }
                    }
                } else {
                    eprintln!("No input provided for DeviceState::Complete");
                    Ok(DeviceState::Complete)
                }
            }
        }
    }
}

