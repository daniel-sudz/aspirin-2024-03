use super::libserial::serial::Serial;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicI32};
use anyhow::Result;
use std::sync::{mpsc, Arc};
use parking_lot::RwLock;
use std::thread;
use std::{io, io::Write};


pub struct BufferedBackgroundSerial {
    serial: Arc<RwLock<Serial>>,
    join_handle: Option<thread::JoinHandle<()>>,
    enable: Arc<AtomicBool>,
    pos: Arc<RwLock<(i32, i32)>>,
}

#[derive(Debug)]
pub struct ButtonStates {
    top_left: bool,
    top_right: bool,
    bottom_left: bool,
    bottom_right: bool,
}

impl BufferedBackgroundSerial {
    /// Writes a message to the tx_buffer
    pub fn send(&self, message: String) -> Result<()> {
        self.serial.write().send(message)?;
        thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    }

    pub fn get_pos(&self) -> (i32, i32) {
        *self.pos.read()
    }

    fn button_states_from_message(message: &String) -> ButtonStates {
        let raw = message.parse::<u8>().unwrap();
        ButtonStates {
            top_left: (raw & (1 << 0)) != 0,
            top_right: (raw & (1 << 1)) != 0,
            bottom_left: (raw & (1 << 2)) != 0,
            bottom_right: (raw & (1 << 3)) != 0,
        }
    }

    fn button_rising_edge(last_states: &ButtonStates, current_states: &ButtonStates) -> ButtonStates {
        ButtonStates {
            top_left: last_states.top_left && !current_states.top_left,
            top_right: last_states.top_right && !current_states.top_right,
            bottom_left: last_states.bottom_left && !current_states.bottom_left,
            bottom_right: last_states.bottom_right && !current_states.bottom_right,
        }
    }

    pub fn update(pos: &Arc<RwLock<(i32, i32)>>, last_states: &ButtonStates, current_states: &ButtonStates) {
        let rising_edges = Self::button_rising_edge(last_states, current_states);

        //println!("last_states: {:?}, current_states: {:?}, rising_edges: {:?}", last_states, current_states, rising_edges);

        if rising_edges.top_left {
            pos.write().0 -= 1;
            pos.write().1 += 1;
        }
        if rising_edges.top_right {
            pos.write().0 -= 1;
            pos.write().1 -= 1;
        }
        if rising_edges.bottom_left {
            pos.write().0 += 1;
            pos.write().1 += 1;
        }
        if rising_edges.bottom_right {
            pos.write().0 += 1;
            pos.write().1 -= 1;
        }
    }

    /// Creates a new BufferedBackgroundSerial instance from a Serial instance
    pub fn from_serial(serial: Serial) -> Self {
        let mut s = Self {
            serial: Arc::new(RwLock::new(serial)),
            enable: Arc::new(AtomicBool::new(true)),
            pos: Arc::new(RwLock::new((0, 0))),
            join_handle: None,
        };

        let serial = s.serial.clone(); 
        let enable = s.enable.clone();
        let pos = s.pos.clone();
        let join_handle = {
            thread::Builder::new().name("buffered_background_serial".to_string()).spawn(move || {
                let mut last_states = ButtonStates {
                    top_left: false,
                    top_right: false,
                    bottom_left: false,
                    bottom_right: false,
                };

                loop {
                    // exit thread if enable is false
                    if !enable.load(std::sync::atomic::Ordering::Relaxed) {
                        break;
                    }

                    // check to see if we have something to receive
                    let received_data = serial.read().receive();
                    match received_data {
                    Ok(data) => {
                        if data.bytes().len() == 1 || data.bytes().len() == 2 {
                            let current_states = Self::button_states_from_message(&data);
                            Self::update(&pos, &last_states, &current_states);
                            println!("pos inside: {:?}", pos);
                            last_states = current_states;
                        }
                    }
                    Err(e) => {
                        println!("Error receiving data: {}", e);
                        }
                    }
                }
            })
        }.unwrap();
        s.join_handle = Some(join_handle);
        s
    }
}

impl Drop for BufferedBackgroundSerial {
    fn drop(&mut self) {
        self.enable.store(false, std::sync::atomic::Ordering::Relaxed);
        if let Some(handle) = self.join_handle.take() {
            handle.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
}
