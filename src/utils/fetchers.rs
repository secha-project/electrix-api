use crate::{
    data::{
        device::Device,
        device_data::{DeviceData, DATA_POINTS},
        event::DeviceEvent,
        event_item::{DeviceEventItem, DeviceEventItemWithId},
        event_item_with_id::EventItemWithId,
        event_mapped_item::{DeviceEventMappedItem, DeviceEventMappedItemWithId},
        host::Host,
        voltage_anomaly::VoltageAnomaly,
    },
    utils::{environ::get_env_or_default, http::get_data}
};


pub async fn get_devices(host: &Host) -> Result<Vec<Device>, String> {
    const DEVICE_ENDPOINT: &str = "/api/v2/meters/";

    get_data(
        host.get_url() + DEVICE_ENDPOINT,
        host.get_headers(),
        vec![],
        host.allow_invalid_certs()
    ).await
}

pub async fn get_device_data(host: &Host, device: &Device, date: &str) -> Result<Vec<DeviceData>, String> {
    const DATA_ENDPOINT: &str = "/api/v2/measurements/";

    get_data(
    host.get_url() + DATA_ENDPOINT,
        host.get_headers(),
        vec![
            ("meter".to_string(), device.id.to_string()),
            ("start".to_string(), date.to_owned() + "T00:00:00"),
            ("end".to_string(), date.to_owned() + "T23:59:59"),
            ("fields".to_string(), DATA_POINTS.to_string()),
        ],
        host.allow_invalid_certs()
    ).await
}

pub async fn get_device_events(host: &Host, device: &Device, date: &str) -> Result<Vec<DeviceEvent>, String> {
    const EVENT_ENDPOINT: &str = "/api/v2/events/";

    get_data(
        host.get_url() + EVENT_ENDPOINT,
        host.get_headers(),
        vec![
            ("meter".to_string(), device.id.to_string()),
            ("start".to_string(), date.to_owned() + "T00:00:00"),
            ("end".to_string(), date.to_owned() + "T23:59:59"),
        ],
        host.allow_invalid_certs()
    ).await
}

pub async fn get_event_data(host: &Host, event_id: u32) -> Result<EventItemWithId, String> {
    const EVENT_DATA_MAPPING: &str = "EVENT_DATA_MAPPING";
    let event_details_endpoint: String = format!("/api/v2/events/{event_id}/");
    let use_mapping: bool = get_env_or_default(EVENT_DATA_MAPPING, &false);
    let params = vec![("map_variable_names".to_string(), use_mapping.to_string())];

    if use_mapping {
        get_data::<DeviceEventMappedItem>(
            host.get_url() + event_details_endpoint.as_str(),
            host.get_headers(),
            params,
            host.allow_invalid_certs()
        )
            .await
            .map(
                |event_item|
                EventItemWithId::Mapped(DeviceEventMappedItemWithId::from_event(event_id, event_item))
            )
    }
    else {
        get_data::<DeviceEventItem>(
            host.get_url() + event_details_endpoint.as_str(),
            host.get_headers(),
            params,
            host.allow_invalid_certs()
        )
            .await
            .map(
                |event_item|
                EventItemWithId::Unmapped(DeviceEventItemWithId::from_event(event_id, event_item))
            )
    }
}

pub async fn get_anomaly_data(host: &Host, device: &Device, date: &str) -> Result<Vec<VoltageAnomaly>, String> {
    const ANOMALY_ENDPOINT: &str = "/api/v2/ssstamps/";

    get_data(
        host.get_url() + ANOMALY_ENDPOINT,
        host.get_headers(),
        vec![
            ("meter".to_string(), device.id.to_string()),
            ("start".to_string(), date.to_owned() + "T00:00:00"),
            ("end".to_string(), date.to_owned() + "T23:59:59"),
        ],
        host.allow_invalid_certs()
    ).await
}
