use socketcan::{CANSocket, CANFrame};
use std::error::Error;
use std::time::Duration;
fn main() -> Result<(), Box<dyn Error>> {
    let socket = CANSocket::open("vcan0")?;
    let frame_id = 0x123;
    let data = [0xde, 0xad, 0xbe, 0xef];
    let frame = CANFrame::new(frame_id, &data, false, false)?;

    println!("Client is sending a frame...");

    socket.write_frame(&frame)?;

    println!("Sent frame: {:?}", frame);

    std::thread::sleep(Duration::from_secs(1));

    Ok(())
}
