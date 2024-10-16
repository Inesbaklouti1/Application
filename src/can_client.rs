use socketcan_isotp::{IsoTpSocket, Id,StandardId};
use std::{error::Error,borrow::Cow,fs};
use std::time::Duration;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub client : ClientConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    pub tx_id: u32,
    pub rx_id: u32,
    pub message: String,
    pub interface : String,
}



fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config(std::env::args().nth(1))?;
    let rx_id = create_id(config.client.rx_id)?;
    let tx_id = create_id(config.client.tx_id)?;
    let mut socket = create_socket(&config.client.interface, rx_id, tx_id)?;
    send_message(&mut socket, &config.client.message)
    
}

fn load_config(path: Option<String>) -> Result<Config, Box<dyn Error>>{
    let default_path = "../config.json";
    let path = path.unwrap_or_else(|| default_path.to_string());
    let config_data = fs::read_to_string(path)?;
    let config: Config = serde_json::from_str(&config_data)?;
    Ok(config)
}

fn create_id(raw: u32) -> Result<Id, Box<dyn Error>> {
    let raw_u16 = raw as u16;
    Ok(Id::Standard(StandardId::new(raw_u16).unwrap()))
}

fn create_socket(ifname: &str, rx_id: Id, tx_id: Id) -> Result<IsoTpSocket, Box<dyn Error>> {
    Ok(IsoTpSocket::open(ifname, rx_id, tx_id)?)
}
fn send_message(socket: &mut IsoTpSocket, message:&str) -> Result<(),Box<dyn Error>>{
    let data = message.as_bytes();

    println!("Client is sending ...");

    socket.write(&data)?;

    println!("Sent frame: {:?}", message);

    std::thread::sleep(Duration::from_secs(1));

    Ok(())
}