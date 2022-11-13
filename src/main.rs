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

    let (stop_receive_tx, stop_receive_rx) = channel();
    let (stop_export_tx, stop_export_rx) = channel();
    let (height_tx, height_rx) = channel();

    let receiver_handle = thread::spawn(move || {
        loop {
            if stop_receive_rx.try_recv().is_ok() {
                break;
            }
            let packet = match flexi_connection.read_packet() {
                Ok(data) => {
                    data
                }
                Err(serial::Error::DeviceTurnedOffError) => {
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
            height_tx.send(height).unwrap();
        }
    });

    let export_thread = thread::spawn(move || {
        let mut height = f64::NAN;
        loop {
            if stop_export_rx.recv().is_ok() {
                break;
            }
            if let Some(h) = height_rx.try_iter().last() {
                height = h;
            }
            println!("height: {}", height);
            thread::sleep(Duration::from_millis(500));
        }
    });

    sleep(Duration::from_secs(30));
    stop_receive_tx.send(()).unwrap();
    stop_export_tx.send(()).unwrap();
    receiver_handle.join().unwrap();
    export_thread.join().unwrap();
}
