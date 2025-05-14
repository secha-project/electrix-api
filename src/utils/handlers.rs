use crate::data::{
    device::Device,
    device_data::DeviceData,
    event::DeviceEvent,
    host::Host,
    voltage_anomaly::VoltageAnomaly
};

pub fn get_host() -> Host {
    match Host::get_from_env() {
        Ok(host) => host,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

pub async fn get_devices(host: &Host) -> Vec<Device> {
    match crate::utils::fetchers::get_devices(host).await {
        Ok(devices) => devices,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

pub async fn get_device_data(host: &Host, device: &Device, date: &str) -> Vec<DeviceData> {
    match crate::utils::fetchers::get_device_data(host, device, date).await {
        Ok(device_data) => device_data,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

pub async fn get_device_events(host: &Host, device: &Device, date: &str) -> Vec<DeviceEvent> {
    match crate::utils::fetchers::get_device_events(host, device, date).await {
        Ok(device_events) => device_events,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

pub async fn get_anomaly_data(host: &Host, device: &Device, date: &str) -> Vec<VoltageAnomaly> {
    match crate::utils::fetchers::get_anomaly_data(host, device, date).await {
        Ok(anomalies) => anomalies,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
