use super::libserial::serial::Serial;
use anyhow::Result;
use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::sync::{mpsc, Arc};
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
            thread::Builder::new()
                .name("buffered_background_serial".to_string())
                .spawn(move || {
                    loop {
                        // exit thread if enable is false
                        if !enable.load(std::sync::atomic::Ordering::Relaxed) {
                            break;
                        }

                        // check to see if we have something to receive
                        let received_data = serial.read().receive();
                        match received_data {
                            Ok(data) => {
                                if data.contains(",") {
                                    let split = data.split(",").collect::<Vec<&str>>();
                                    if split.len() == 2 {
                                        let x = split[0].parse::<i32>().unwrap();
                                        let y = split[1].parse::<i32>().unwrap();
                                        *pos.write() = (x, y);
                                        println!("pos inside: {:?}", pos);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Error receiving data: {}", e);
                            }
                        }
                    }
                })
        }
        .unwrap();
        s.join_handle = Some(join_handle);
        s
    }
}

impl Drop for BufferedBackgroundSerial {
    fn drop(&mut self) {
        self.enable
            .store(false, std::sync::atomic::Ordering::Relaxed);
        if let Some(handle) = self.join_handle.take() {
            handle.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {}
