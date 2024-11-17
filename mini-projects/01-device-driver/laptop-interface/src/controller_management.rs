use crate::commander::Commander;
use crate::libserial::list_ports::get_all_rpi_ports;
use crate::libserial::types::Port;

use super::libserial::serial::Serial;
use anyhow::Result;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
pub enum DeviceState {
    PendingInit = 0,
    PendingStart = 1,
    Running = 2,
    Complete = 3,
}

#[derive(PartialEq, Clone)]
pub enum ControllerInput {
    Reset,
    Restart,
    StartController,
    StopGame,
}

pub struct MultiDevice {
    state: DeviceState,
    commanders: Vec<Commander>,
}

pub struct BackgroundMultiDevice {
    state: Arc<RwLock<DeviceState>>,
    pos: Arc<RwLock<Vec<(i32, i32)>>>,
    controller_input: Arc<RwLock<Option<ControllerInput>>>,
    thread: Option<thread::JoinHandle<()>>,
    enable: Arc<AtomicBool>,
}

impl BackgroundMultiDevice {
    pub fn get_state(&self) -> DeviceState {
        *self.state.read().unwrap()
    }   
    pub fn get_pos(&self) -> Vec<(i32, i32)> {
        self.pos.read().unwrap().clone()
    }       
    pub fn set_controller_input(&self, controller_input: Option<ControllerInput>) {
        *self.controller_input.write().unwrap() = controller_input;
    }   
    pub fn from_auto_configure(num_devices: usize) -> Result<Self> {
        let mut multi_device = MultiDevice::from_auto_configure(num_devices)?;
        let state = Arc::new(RwLock::new(multi_device.state));
        let controller_input = Arc::new(RwLock::new(None));
        let pos = Arc::new(RwLock::new(vec![(0,0); num_devices]));
        let enable: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));
        let mut background_multidevice = BackgroundMultiDevice { state, pos, controller_input, enable, thread: None };

        let state = background_multidevice.state.clone();
        let pos = background_multidevice.pos.clone();
        let controller_input = background_multidevice.controller_input.clone();
        let enable = background_multidevice.enable.clone();
        let thread = thread::spawn(move || {
            loop {
                if enable.load(std::sync::atomic::Ordering::Relaxed) { 
                    let controller_input_value = controller_input.read().unwrap().clone();
                    multi_device.state_action(&controller_input_value).unwrap();
                    *state.write().unwrap() = multi_device.get_state();
                    *pos.write().unwrap() = multi_device.get_pos();
                } else {
                    break;
                }
            }
        });

        background_multidevice.thread = Some(thread);
        Ok(background_multidevice)
    }
}
impl Drop for BackgroundMultiDevice {
    fn drop(&mut self) {
        self.enable.store(false, std::sync::atomic::Ordering::Relaxed);
        if let Some(handle) = self.thread.take() {
            handle.join().unwrap();
        }
    }
}

impl MultiDevice {
    pub fn from_auto_configure(num_devices: usize) -> Result<Self> {
        let ports = get_all_rpi_ports();
        let commanders = match ports.len().cmp(&num_devices) {
            std::cmp::Ordering::Equal => ports.into_iter().map(|port| Commander::from_port(port)).collect(),
            std::cmp::Ordering::Less => Err(anyhow::anyhow!("Not enough devices found - need {}, found {}", num_devices, ports.len())),
            std::cmp::Ordering::Greater => Err(anyhow::anyhow!("Too many devices found - need {}, found {}", num_devices, ports.len()))
        }?;
        Ok(Self { state: DeviceState::PendingInit, commanders })
    }

    pub fn state_action(
        &mut self,
        input: &Option<ControllerInput>,
    ) -> Result<()> {
        self.state = match self.state {
            DeviceState::PendingInit => {
                Commander::run_on_all_commanders(&mut self.commanders, |commander: &mut Commander| commander.transition_to_pending_start())?;
                DeviceState::PendingStart
            }
            DeviceState::PendingStart => {
                if let Some(input_value) = input {  
                    match input_value {
                        ControllerInput::StartController => {
                            Commander::run_on_all_commanders(&mut self.commanders, |commander: &mut Commander| {
                                commander.set_ready_led()?;
                                commander.set_set_led()?;
                                thread::sleep(Duration::from_millis(1000));
                                commander.set_go_led()?;
                                thread::sleep(Duration::from_millis(1000));
                                commander.set_all_leds_off()?;
                                commander.transition_to_running()
                            })?;
                            DeviceState::Running
                        }
                        _ => {DeviceState::PendingStart}
                    }
                } else {
                    DeviceState::PendingStart
                }
            }
            DeviceState::Running => {
                if let Some(input_value) = input {
                    match input_value {
                        ControllerInput::StopGame => {
                            Commander::run_on_all_commanders(&mut self.commanders, |commander: &mut Commander| commander.transition_to_complete())?;
                            DeviceState::Complete
                        }
                        _ => {DeviceState::Running}
                    }
                } else {
                    DeviceState::Running
                }
            }
            DeviceState::Complete => {
                if let Some(input_value) = input {
                    match input_value {
                        ControllerInput::Reset => {
                            Commander::run_on_all_commanders(&mut self.commanders, |commander: &mut Commander| commander.transition_to_pending_init_from_complete())?;
                            DeviceState::PendingInit
                        }
                        ControllerInput::Restart => {
                            Commander::run_on_all_commanders(&mut self.commanders, |commander: &mut Commander| commander.transition_to_pending_start_from_complete())?;
                            DeviceState::PendingStart
                        }
                        ControllerInput::StartController => {
                            Commander::run_on_all_commanders(&mut self.commanders, |commander: &mut Commander| commander.transition_to_running_from_complete())?;
                            DeviceState::Running
                        },
                        _ => {DeviceState::Complete}
                    }
                } else {
                    eprintln!("No input provided for DeviceState::Complete");
                    DeviceState::Complete
                }
            }
        };
        Ok(())
    }

    pub fn get_pos(&self) -> Vec<(i32, i32)> {
        self.commanders.iter().map(|commander| commander.get_pos()).collect()
    }

    pub fn get_state(&self) -> DeviceState {
        self.state.clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_device() {
        let multi_device = MultiDevice::from_auto_configure(2).unwrap();
    }

}