use std::process::Command;
fn main(){
    Command::new("sh")
    .arg("../vcan_creation.sh")
    .status()
    .expect("Failed to execute script");

}