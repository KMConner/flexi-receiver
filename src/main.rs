extern crate core;

mod digit;
mod serial;

fn main() {
    let serial_connection = serialport::new("/dev/ttyS0", 9600).open().unwrap();
    let mut flexi_connection = serial::FlexiConnection::new(serial_connection);
    loop {
        let packet = match flexi_connection.read_packet() {
            Ok(data) => {
                data
            }
            Err(serial::Error::DeviceTurnedOffError)=>{
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
}
