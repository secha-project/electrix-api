use serde::Deserialize;

use super::{event_mapped_data::DeviceEventMappedData, event_settings::DeviceEventSettings};


#[derive(Clone, Debug, Deserialize)]
pub struct DeviceEventMappedItem {
    pub settings: DeviceEventSettings,
    pub data: Vec<DeviceEventMappedData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceEventMappedItemWithId {
    pub id: u32,
    pub settings: DeviceEventSettings,
    pub data: Vec<DeviceEventMappedData>,
}


impl DeviceEventMappedItemWithId {
    pub fn from_event(event_id: u32, event_item: DeviceEventMappedItem) -> Self {
        Self {
            id: event_id,
            settings: event_item.settings,
            data: event_item.data,
        }
    }

    pub fn to_header_record() -> Vec<String> {
        [
            "meter".to_string(),
            "event_id".to_string(),
        ]
            .iter()
            .chain(&DeviceEventMappedData::to_header_record())
            .cloned()
            .collect()
    }

    pub fn to_data_records(&self, current_factor: f32, voltage_factor: f32) -> Vec<Vec<String>> {
        self.data
            .iter()
            .map(|data| {
                [
                    vec![
                        self.settings.meter.to_string(),
                        self.id.to_string(),
                    ],
                    data.to_record(current_factor, voltage_factor),
                ]
                    .concat()
            })
            .collect()
    }
}
