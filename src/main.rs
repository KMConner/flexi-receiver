extern crate core;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use metrics::{gauge, register_gauge};
use metrics_exporter_prometheus::PrometheusBuilder;

mod digit;
mod serial;

fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 8080);
    register_gauge!("desk_height");
    PrometheusBuilder::new().with_http_listener(addr).install().unwrap();

    let serial_connection = serialport::new("/dev/ttyS0", 9600).open().unwrap();
    let mut flexi_connection = serial::FlexiConnection::new(serial_connection);

    loop {
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
        let new_height = match digit::parse(&digit_bin) {
            Ok(h) => h,
            Err(e) => {
                println!("{:?}", e);
                continue;
            }
        };
        gauge!("desk_height", new_height as f64);
    }
}
