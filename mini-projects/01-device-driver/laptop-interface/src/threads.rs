use super::libserial::serial::Serial;
use std::collections::VecDeque;
use std::sync::atomic::AtomicBool;
use anyhow::Result;
use std::sync::{mpsc, Arc, RwLock};
use std::thread;
use std::{io, io::Write};


pub struct BufferedBackgroundSerial {
    serial: Arc<RwLock<Serial>>,
    rx_buffer: Arc<RwLock<VecDeque<String>>>,
    join_handle: Option<thread::JoinHandle<()>>,
    enable: Arc<AtomicBool>,
}

impl BufferedBackgroundSerial {
    /// Writes a message to the tx_buffer
    pub fn send(&self, message: String) -> Result<()> {
        self.serial.write().unwrap().send(message)
    }

    pub fn receive(&self) -> Result<String> {
        self.rx_buffer.write().unwrap().pop_front().ok_or(anyhow::anyhow!("No data to receive"))
    }

    /// Creates a new BufferedBackgroundSerial instance from a Serial instance
    pub fn from_serial(serial: Serial) -> Self {
        let mut s = Self {
            serial: Arc::new(RwLock::new(serial)),
            rx_buffer: Arc::new(RwLock::new(VecDeque::new())),
            enable: Arc::new(AtomicBool::new(true)),
            join_handle: None,
        };

        let serial = s.serial.clone(); 
        let rx_buffer = s.rx_buffer.clone();
        let enable = s.enable.clone();
        let join_handle = unsafe {
            thread::Builder::new().name("buffered_background_serial".to_string()).spawn_unchecked(move || {
            loop {
                // exit thread if enable is false
                if !enable.load(std::sync::atomic::Ordering::Relaxed) {
                    break;
                }
                // check to see if we have something to receive
                let received_data = serial.read().unwrap().receive();
                match received_data {
                    Ok(data) => {
                        let mut rx_buffer = rx_buffer.write().unwrap();
                        rx_buffer.push_back(data);
                    }
                    Err(e) => {
                        println!("Error receiving data: {}", e);
                    }
                }
            }
            })
            .unwrap()
        };
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
