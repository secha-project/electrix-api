mod data;
mod utils;

use crate::data::data_structs::{Device, Host};
use crate::utils::http_utils::get_devices;


#[tokio::main]
async fn main() {
    let host: Host = match Host::get_from_env() {
        Ok(host) => host,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    let devices: Vec<Device> = match get_devices(host).await {
        Ok(devices) => devices,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    println!("Found {} devices", devices.len());
    let device_info = devices
        .iter()
        .map(data::data_structs::Device::pretty_print)
        .fold(String::new(), |acc, e| acc + &e);
    print!("{device_info}");
}
