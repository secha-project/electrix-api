mod data;
mod utils;

use std::collections::HashMap;
use std::env;

use data::{
    device::Device,
    device_data::DeviceData,
    event::DeviceEvent,
    event_item::DeviceEventItemWithId,
    event_settings::DeviceEventSettings,
    host::Host,
    voltage_anomaly::VoltageAnomaly,
};
use utils::{
    csv_writers::{write_anomaly_data, write_device_data, write_device_events, write_devices, write_event_data, write_event_triggers},
    environ::get_env_or_default,
    fetchers::get_event_data,
    handlers::{get_anomaly_data, get_device_data, get_device_events, get_devices},
};


#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <date>", args[0]);
        std::process::exit(1);
    }
    let date: String = args[1].clone();
    let verbose: bool = get_env_or_default("VERBOSE", &false);

    let host: Host = utils::handlers::get_host();

    let devices: Vec<Device> = get_devices(&host).await;
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
    let mut anomaly_collection: Vec<VoltageAnomaly> = vec![];

    for device in devices {
        if verbose {
            println!("\n{}", device.pretty_print());
        }
        let device_data: Vec<DeviceData> = get_device_data(&host, &device, &date).await;
        device_data_collection.extend(device_data.clone());

        println!("Found {} data points for device {} on date {}", device_data.len(), device.id, date);
        if verbose {
            if let Some(data) = device_data.first() {
                println!("First data point:");
                println!("{}", data.pretty_print());
            }
        }

        let device_events: Vec<DeviceEvent> = get_device_events(&host, &device, &date).await;
        device_event_collection.extend(device_events.clone());

        println!("Found {} events for device {} on date {}", device_events.len(), device.id, date);
        let mut event_printed: bool = !verbose;
        for event in device_events {
            match get_event_data(&host, event.id).await {
                Ok(event_item) => {
                    let event_item_clone = event_item.clone();
                    if !event_triggers.contains(&event_item_clone.settings) {
                        event_triggers.push(event_item_clone.clone().settings);
                    }
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
            }
        }

        let anomalies: Vec<VoltageAnomaly> = get_anomaly_data(&host, &device, &date).await;
        anomaly_collection.extend(anomalies.clone());

        println!("Found {} voltage swags/swells for device {} on date {}", anomalies.len(), device.id, date);
        if verbose {
            if let Some(anomaly) = anomalies.first() {
                println!("First voltage swag/swell:");
                println!("{}", anomaly.pretty_print());
            }
        }

    }

    write_device_data(&device_map, &device_data_collection, &date);
    write_device_events(&device_event_collection, &date);
    write_event_triggers(&event_triggers, &date);
    write_event_data(&device_map, &event_data_collection.clone(), &date);
    write_anomaly_data(&device_map, &anomaly_collection, &date);
}
