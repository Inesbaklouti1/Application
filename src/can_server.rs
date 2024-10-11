use socketcan::CANSocket;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let socket = CANSocket::open("vcan0")?;

    println!("Server is listening on vcan0...");

    loop {
        match socket.read_frame() {
            Ok(frame) => {
                println!("Received CAN frame: {:?}", frame);
            },
            Err(e) => {
                eprintln!("Failed to read CAN frame: {}", e);
            }
        }
    }
}
