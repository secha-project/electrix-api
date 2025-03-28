use std::collections::HashMap;
use serde::Deserialize;

use super::{event_data::DeviceEventData, event_settings::DeviceEventSettings};


#[derive(Clone, Debug, Deserialize)]
pub struct DeviceEventItem {
    pub settings: DeviceEventSettings,
    pub data: Vec<DeviceEventData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceEventItemWithId {
    pub id: u32,
    pub settings: DeviceEventSettings,
    pub data: Vec<DeviceEventData>,
}


impl DeviceEventItemWithId {
    pub fn from_event(event_id: u32, event_item: DeviceEventItem) -> Self {
        Self {
            id: event_id,
            settings: event_item.settings,
            data: event_item.data,
        }
    }

    fn values_to_maps(&self, event_id: u32) -> Vec<HashMap<String, String>> {
        self.data
            .iter()
            .map(|data| HashMap::from([
                ("event_id".to_string(), event_id.to_string()),
                ("timestamp".to_string(), format!("{}Z", data.timestamp.clone())),
                (self.settings.d1.clone(), data.d1.to_string()),
                (self.settings.d2.clone(), data.d2.to_string()),
                (self.settings.d3.clone(), data.d3.to_string()),
                (self.settings.d4.clone(), data.d4.to_string()),
                (self.settings.d5.clone(), data.d5.to_string()),
                (self.settings.d6.clone(), data.d6.to_string()),
                (self.settings.d7.clone(), data.d7.to_string()),
                (self.settings.d8.clone(), data.d8.to_string()),
                (self.settings.d9.clone(), data.d9.to_string()),
                (self.settings.d10.clone(), data.d10.to_string()),
                (self.settings.d11.clone(), data.d11.to_string()),
                (self.settings.d12.clone(), data.d12.to_string()),
                (self.settings.d13.clone(), data.d13.to_string()),
                (self.settings.d14.clone(), data.d14.to_string()),
                (self.settings.d15.clone(), data.d15.to_string()),
                (self.settings.d16.clone(), data.d16.to_string()),
                (self.settings.d17.clone(), data.d17.to_string()),
                (self.settings.d18.clone(), data.d18.to_string()),
                (self.settings.d19.clone(), data.d19.to_string()),
                (self.settings.d20.clone(), data.d20.to_string()),
                (self.settings.d21.clone(), data.d21.to_string()),
                (self.settings.d22.clone(), data.d22.to_string()),
                (self.settings.d23.clone(), data.d23.to_string()),
                (self.settings.d24.clone(), data.d24.to_string()),
                (self.settings.d25.clone(), data.d25.to_string()),
                (self.settings.d26.clone(), data.d26.to_string()),
                (self.settings.d27.clone(), data.d27.to_string()),
                (self.settings.d28.clone(), data.d28.to_string()),
            ]))
            .collect()
    }

    pub fn to_header_record(measurement_types: &[String]) -> Vec<String> {
        [
            "meter".to_string(),
            "event_id".to_string(),
            "timestamp".to_string(),
        ]
            .iter()
            .chain(measurement_types.iter())
            .cloned()
            .collect()
    }

    pub fn to_data_records(&self, event_id: u32, measurement_types: &[String]) -> Vec<Vec<String>> {
        self.values_to_maps(event_id)
            .iter()
            .map(|data_map| {
                [
                    self.settings.meter.to_string(),
                    data_map.get("event_id").unwrap().to_string(),
                    format!("{}Z", data_map.get("timestamp").unwrap().to_string()),
                ]
                    .iter()
                    .cloned()
                    .chain(
                        measurement_types
                        .iter()
                        .map(
                            |measurement_type|
                                data_map
                                    .get(measurement_type)
                                    .map_or_else(String::new, std::string::ToString::to_string)
                        )
                    )
                    .collect()
            })
            .collect()
        }
}
