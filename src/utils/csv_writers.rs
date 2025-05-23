use csv;
use std::collections::HashMap;

use crate::data::event_item::DeviceEventItemWithId;
use crate::data::{
    device::Device,
    device_data::DeviceData,
    event::DeviceEvent,
    event_settings::DeviceEventSettings,
    voltage_anomaly::VoltageAnomaly
};

const DATA_FOLDER: &str = "data";


fn write_records(filename: &str, records: Vec<Vec<String>>) {
    let mut writer = match csv::Writer::from_path(filename) {
        Ok(writer) => writer,
        Err(err) => {
            eprintln!("Cannot create file ({filename}): {err}");
            return;
        }
    };

    for record in records {
        match writer.write_record(record) {
            Ok(()) => (),
            Err(err) => {
                eprintln!("Cannot write record: {err}");
                return;
            }
        }
    }

    match writer.flush() {
        Ok(()) => (),
        Err(err) => eprintln!("Cannot flush writer: {err}"),
    }
}

pub fn write_devices(devices: &[Device], date: &str) {
    let filename: String = format!("{DATA_FOLDER}/{date}_devices.csv");

    let records: Vec<Vec<String>> = std::iter::once(Device::to_header_record())
        .chain(devices.iter().map(Device::to_record))
        .collect();

    write_records(&filename, records);
}

pub fn write_device_data(devices: &HashMap<u32, Device>, device_data: &[DeviceData], date: &str) {
    let filename: String = format!("{DATA_FOLDER}/{date}_data.csv");

    let records: Vec<Vec<String>> = std::iter::once(DeviceData::to_header_record())
        .chain(
            device_data
                .iter()
                .map(|data| {
                    let device: &Device = if let Some(device) = devices.get(&data.meter) {
                        device
                    }
                    else {
                        eprintln!("Device {} not found", data.meter);
                        return vec![];
                    };
                    data.to_record(device.ik, device.uk)
                })
        )
        .collect();

    write_records(&filename, records);
}

pub fn write_device_events(events: &[DeviceEvent], date: &str) {
    let filename: String = format!("{DATA_FOLDER}/{date}_events.csv");

    let records: Vec<Vec<String>> = std::iter::once(DeviceEvent::to_header_record())
        .chain(events.iter().map(DeviceEvent::to_record))
        .collect();

    write_records(&filename, records);
}

pub fn write_event_triggers(events: &[DeviceEventSettings], date: &str) {
    let filename: String = format!("{DATA_FOLDER}/{date}_event_triggers.csv");

    let records: Vec<Vec<String>> = std::iter::once(DeviceEventSettings::to_header_record())
        .chain(events.iter().map(DeviceEventSettings::to_record))
        .collect();

    write_records(&filename, records);
}

pub fn write_event_data(devices: &HashMap<u32, Device>, events: &[DeviceEventItemWithId], date: &str) {
    let filename: String = format!("{DATA_FOLDER}/{date}_event_data.csv");

    let mut measurement_types: Vec<String> = vec![];
    for event in events {
        for measurement_type in event.settings.measurement_types() {
            if !measurement_types.contains(&measurement_type) {
                measurement_types.push(measurement_type);
            }
        }
    }

    let records: Vec<Vec<String>> = std::iter::once(&DeviceEventItemWithId::to_header_record(&measurement_types))
        .cloned()
        .chain(
            events
                .iter()
                .flat_map(|event| {
                    let device: &Device = if let Some(device) = devices.get(&event.settings.meter) {
                        device
                    }
                    else {
                        eprintln!("Device {} not found", event.settings.meter);
                        return vec![];
                    };

                    event.to_data_records(event.id, &measurement_types, device.ik, device.uk)
        })
        )
        .collect();

    write_records(&filename, records);
}

pub fn write_anomaly_data(devices: &HashMap<u32, Device>, anomalies: &[VoltageAnomaly], date: &str) {
    let filename: String = format!("{DATA_FOLDER}/{date}_anomaly_data.csv");

    let records: Vec<Vec<String>> = std::iter::once(VoltageAnomaly::to_header_record())
        .chain(
            anomalies
                .iter()
                .map(|anomaly| {
                    let device: &Device = if let Some(device) = devices.get(&anomaly.meter) {
                        device
                    }
                    else {
                        eprintln!("Device {} not found", anomaly.meter);
                        return vec![];
                    };
                    anomaly.to_record(device.ik)
                })
        )
        .collect();

    write_records(&filename, records);
}
