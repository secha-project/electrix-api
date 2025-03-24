mod data;
mod utils;

use std::collections::HashMap;
use std::env;

use data::event_item::DeviceEventItemWithId;
use data::{
    device::Device,
    device_data::DeviceData,
    event::DeviceEvent,
    event_settings::DeviceEventSettings,
    host::Host
};
use utils::csv_writers::{write_device_data, write_device_events, write_devices, write_event_data, write_event_triggers};
use utils::environ::get_env_or_default;
use utils::fetchers::{get_devices, get_device_data, get_device_events, get_event_data};


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <date>", args[0]);
        std::process::exit(1);
    }
    let date: String = args[1].clone();
    let verbose: bool = get_env_or_default("VERBOSE", &false);

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
    write_devices(&devices, &date);

    let device_map: HashMap<u32, Device> = devices
        .clone()
        .into_iter()
        .map(|device| (device.id, device))
        .collect();
    let mut device_data_collection: Vec<DeviceData> = vec![];
    let mut device_event_collection: Vec<DeviceEvent> = vec![];
    let mut event_triggers: Vec<DeviceEventSettings> = vec![];
    let mut event_data_collection: Vec<DeviceEventItemWithId> = vec![];

    for device in devices {
        if verbose {
            println!("{}", device.pretty_print());
        }
        let device_data: Vec<DeviceData> = match get_device_data(&host, &device, &date).await {
            Ok(device_data) => device_data,
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        };

        device_data_collection.extend(device_data.clone());

        println!("Found {} data points for device {} on date {}", device_data.len(), device.id, date);
        if verbose {
            if let Some(data) = device_data.first() {
                println!("First data point:");
                println!("{}", data.pretty_print());
            }
        }

        let device_events: Vec<DeviceEvent> = match get_device_events(&host, &device, &date).await {
            Ok(device_events) => device_events,
            Err(error) => {
                eprintln!("{error}");
                std::process::exit(1);
            }
        };

        device_event_collection.extend(device_events.clone());

        println!("Found {} events for device {} on date {}", device_events.len(), device.id, date);
        let mut event_printed: bool = !verbose;
        for event in device_events {
            match get_event_data(&host, event.id).await {
                Ok(event_item) => {
                    let event_item_clone = event_item.clone();
                    if !event_triggers.contains(&event_item_clone.settings) {
                        event_triggers.push(event_item_clone.clone().settings);
                    };
                    event_data_collection.push(event_item_clone);

                    if !event_printed {
                        println!("{}", event.pretty_print_with_details(&event_item));
                        event_printed = true;
                    }
                },
                Err(error) => {
                    eprintln!("{error}");
                    if !event_printed {
                        println!("{}", event.pretty_print());
                        event_printed = true;
                    }
                },
            };
        }
    }

    write_device_data(&device_map, &device_data_collection, &date);
    write_device_events(&device_event_collection, &date);
    write_event_triggers(&event_triggers, &date);
    write_event_data(&event_data_collection.clone(), &date);
}
