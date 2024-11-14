use super::libserial::serial::Serial;
use std::sync::mpsc;
use std::thread;
use std::{io, io::Write};

pub fn parallel_processing() {
    let (tx1, rx1) = mpsc::channel();
    // let (tx2, rx2) = mpsc::channel();

    // Controller 1 thread
    let controller_one = thread::spawn(move || {
        let port = Serial::from_auto_configure().expect("Failed to configure port");
        println!("Serial port configured for controller one");

        loop {
            match port.receive() {
                Ok(data) => {
                    io::stdout().flush().expect("Failed to flush stdout");
                    match tx1.send(data) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Controller one failed to send data: {:?}", e);
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error receiving data from controller one: {:?}", e);
                }
            }
        }
    });

    // Controller 2 thread
    // let controller_two = thread::spawn(move || {
    //     let port = Serial::from_auto_configure().expect("Failed to configure port");
    //     println!("Serial port configured for controller two");

    //     loop {
    //         match port.receive() {
    //             Ok(data) => {
    //                 io::stdout().flush().expect("Failed to flush stdout");
    //                 match tx2.send(data) {
    //                     Ok(_) => {}
    //                     Err(e) => {
    //                         eprintln!("Controller two failed to send data: {:?}", e);
    //                         break;
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 eprintln!("Error receiving data from controller two: {:?}", e);
    //             }
    //         }
    //     }
    // });

    // Receiver thread that processes all data from both controllers
    let receive_thread = thread::spawn(move || {
        println!("Receiver thread started");
        loop {
            match rx1.recv() {
                Ok(data) => {
                    println!("Controller one input: {}", data);
                }
                Err(e) => {
                    eprintln!("Receiver thread encountered channel error: {:?}", e);
                    break;
                }
            }

            // match rx2.recv() {
            //     Ok(data) => {
            //         println!("Controller two input: {}", data);
            //     }
            //     Err(e) => {
            //         eprintln!("Receiver thread encountered channel error: {:?}", e);
            //         break;
            //     }
            // }
        }
    });

    // Wait for all threads to finish
    controller_one
        .join()
        .expect("Controller one thread panicked");
    // controller_two
    //     .join()
    //     .expect("Controller two thread panicked");
    receive_thread.join().expect("Receiver thread panicked");
}

#[cfg(test)]
mod tests {
    use super::parallel_processing;
    #[test]
    fn test_parallel_processing() {
        parallel_processing();
    }
}
