use socketcan_isotp::{IsoTpSocket, Id, StandardId};
use std::{error::Error,borrow::Cow,fs};
use serde_json::from_reader;
use std::env;
use std::io::BufReader;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub server : ServerConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub tx_id: u32,
    pub rx_id: u32,
    pub interface : String,
}
fn main() -> Result<(), Box<dyn Error>> {
    let config = load_config(std::env::args().nth(1))?;
    let rx_id = create_id(config.server.rx_id)?;
    let tx_id = create_id(config.server.tx_id)?;
    let mut socket = create_socket(&config.server.interface, rx_id, tx_id)?;

    println!("Server is listening ...");

    run_server(&mut socket)
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

fn run_server(socket: &mut IsoTpSocket) -> Result<(), Box<dyn Error>> {
    loop {
        match socket.read() {
            Ok(data) => {
                let message: Cow<'_, str> = String::from_utf8_lossy(data);
                println!("Received frame: {:?}", message);
            }
            Err(e) => {
                eprintln!("Failed to read frame: {}", e);
            }
        }
    }
}
