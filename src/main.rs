mod data;
mod utils;

use std::env;
use self::data::{device::Device, device_data::DeviceData, event::DeviceEvent, host::Host};
use crate::utils::http_utils::{get_devices, get_device_data, get_device_events, get_event_data};


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <date>", args[0]);
        std::process::exit(1);
    }
    let date: String = args[1].clone();

    let host: Host = match Host::get_from_env() {
        Ok(host) => host,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    let devices: Vec<Device> = match get_devices(&host).await {
        Ok(devices) => devices,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    println!("Found {} devices", devices.len());
    for device in devices {
        println!("{}", device.pretty_print());
        let device_data: Vec<DeviceData> = match get_device_data(&host, &device, &date).await {
            Ok(device_data) => device_data,
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        };

        println!("Found {} data points for device {} on date {}", device_data.len(), device.id, date);
        if let Some(data) = device_data.first() {
            println!("First data point:");
            println!("{}", data.pretty_print());
        }

        let device_events: Vec<DeviceEvent> = match get_device_events(&host, &device, &date).await {
            Ok(device_events) => device_events,
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        };

        println!("Found {} events for device {} on date {}", device_events.len(), device.id, date);
        if let Some(event) = device_events.first() {
            println!("First event:");
            match get_event_data(&host, event.id).await {
                Ok(event_data) => {
                    println!("{}", event.pretty_print_with_details(&event_data));
                },
                Err(error) => {
                    eprintln!("{error}");
                    println!("{}", event.pretty_print());
                },
            };
        }
    }
}
