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

pub struct Device {
    state: DeviceState,
}

pub fn controller_state_change(
    current_state: DeviceState,
    input: Option<String>,
) -> Result<DeviceState> {
    let port = Serial::from_auto_configure()?;
    match current_state {
        DeviceState::PendingInit => {
            port.send("init controller".to_string()).unwrap();
            Ok(DeviceState::PendingStart)
        }
        DeviceState::PendingStart => {
            port.send("set ready led".to_string()).unwrap();
            thread::sleep(Duration::from_millis(1000));
            port.send("set set led".to_string()).unwrap();
            thread::sleep(Duration::from_millis(1000));
            port.send("set go led".to_string()).unwrap();
            thread::sleep(Duration::from_millis(1000));
            port.send("clear all leds".to_string()).unwrap();
            Ok(DeviceState::Running)
        }
        DeviceState::Running => {
            port.send("stop controller".to_string()).unwrap();
            Ok(DeviceState::Complete)
        }
        DeviceState::Complete => {
            if let Some(input_value) = input {
                port.send(input_value.clone())?;
                match input_value.as_str() {
                    "reset" => Ok(DeviceState::PendingInit),
                    "restart" => Ok(DeviceState::PendingStart),
                    "start controller" => Ok(DeviceState::Running),
                    _ => {
                        eprintln!("Invalid input for resetting game");
                        Ok(DeviceState::Complete)
                    }
                }
            } else {
                eprintln!("No input provided for DeviceState::Complete");
                Ok(DeviceState::Complete)
            }
        }
    }
}
