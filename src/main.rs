extern crate core;

use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod digit;
mod serial;

fn main() {
    let serial_connection = serialport::new("/dev/ttyS0", 9600).open().unwrap();
    let mut flexi_connection = serial::FlexiConnection::new(serial_connection);

    let (stop_tx, stop_rx) = channel();

    let receiver_handle = thread::spawn(move || {
        loop {
            if stop_rx.try_recv().is_ok() {
                break;
            }
            let packet = match flexi_connection.read_packet() {
                Ok(data) => {
                    data
                }
                Err(serial::Error::DeviceTurnedOffError) => {
                    println!("no data");
                    continue;
                }
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };
            if packet.len() == 0 {
                println!("empty packet");
                continue;
            }

            if packet[0] != 0x12 || packet.len() != 4 {
                continue;
            }

            let digit_bin: [u8; 3] = [packet[1], packet[2], packet[3]];
            let height = match digit::parse(&digit_bin) {
                Ok(h) => h,
                Err(e) => {
                    println!("{:?}", e);
                    continue;
                }
            };
            println!("Current height: {:3.1}", height);
        }
    });

    sleep(Duration::from_secs(10));
    stop_tx.send(()).unwrap();
    receiver_handle.join().unwrap();
}
