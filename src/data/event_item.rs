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

    fn get_measurement_factor(measurement_type: &str, current_factor: f32, voltage_factor: f32) -> f32 {
        match measurement_type {
            "UL1" | "UL2" | "UL3" | "U12" | "U23" | "U31" | "TDUL1" | "TDUL2" | "TDUL3" => voltage_factor,
            "IL1" | "IL2" | "IL3" | "IN" | "TDIL1" | "TDIL2" | "TDIL3" => current_factor,
            "P1L1" | "P1L2" | "P1L3" | "Q1L1" | "Q1L2" | "Q1L3" | "DPFL1" | "DPFL2" | "DPFL3" => voltage_factor * current_factor,
            _ => 1.0,
        }
    }

    fn values_to_maps(&self, event_id: u32, current_factor: f32, voltage_factor: f32) -> Vec<HashMap<String, String>> {
        self.data
            .iter()
            .map(|data| HashMap::from([
                ("event_id".to_string(), event_id.to_string()),
                ("timestamp".to_string(), format!("{}Z", data.timestamp.clone())),
                (self.settings.d1.clone(), (data.d1 * Self::get_measurement_factor(&self.settings.d1, current_factor, voltage_factor)).to_string()),
                (self.settings.d2.clone(), (data.d2 * Self::get_measurement_factor(&self.settings.d2, current_factor, voltage_factor)).to_string()),
                (self.settings.d3.clone(), (data.d3 * Self::get_measurement_factor(&self.settings.d3, current_factor, voltage_factor)).to_string()),
                (self.settings.d4.clone(), (data.d4 * Self::get_measurement_factor(&self.settings.d4, current_factor, voltage_factor)).to_string()),
                (self.settings.d5.clone(), (data.d5 * Self::get_measurement_factor(&self.settings.d5, current_factor, voltage_factor)).to_string()),
                (self.settings.d6.clone(), (data.d6 * Self::get_measurement_factor(&self.settings.d6, current_factor, voltage_factor)).to_string()),
                (self.settings.d7.clone(), (data.d7 * Self::get_measurement_factor(&self.settings.d7, current_factor, voltage_factor)).to_string()),
                (self.settings.d8.clone(), (data.d8 * Self::get_measurement_factor(&self.settings.d8, current_factor, voltage_factor)).to_string()),
                (self.settings.d9.clone(), (data.d9 * Self::get_measurement_factor(&self.settings.d9, current_factor, voltage_factor)).to_string()),
                (self.settings.d10.clone(), (data.d10 * Self::get_measurement_factor(&self.settings.d10, current_factor, voltage_factor)).to_string()),
                (self.settings.d11.clone(), (data.d11 * Self::get_measurement_factor(&self.settings.d11, current_factor, voltage_factor)).to_string()),
                (self.settings.d12.clone(), (data.d12 * Self::get_measurement_factor(&self.settings.d12, current_factor, voltage_factor)).to_string()),
                (self.settings.d13.clone(), (data.d13 * Self::get_measurement_factor(&self.settings.d13, current_factor, voltage_factor)).to_string()),
                (self.settings.d14.clone(), (data.d14 * Self::get_measurement_factor(&self.settings.d14, current_factor, voltage_factor)).to_string()),
                (self.settings.d15.clone(), (data.d15 * Self::get_measurement_factor(&self.settings.d15, current_factor, voltage_factor)).to_string()),
                (self.settings.d16.clone(), (data.d16 * Self::get_measurement_factor(&self.settings.d16, current_factor, voltage_factor)).to_string()),
                (self.settings.d17.clone(), (data.d17 * Self::get_measurement_factor(&self.settings.d17, current_factor, voltage_factor)).to_string()),
                (self.settings.d18.clone(), (data.d18 * Self::get_measurement_factor(&self.settings.d18, current_factor, voltage_factor)).to_string()),
                (self.settings.d19.clone(), (data.d19 * Self::get_measurement_factor(&self.settings.d19, current_factor, voltage_factor)).to_string()),
                (self.settings.d20.clone(), (data.d20 * Self::get_measurement_factor(&self.settings.d20, current_factor, voltage_factor)).to_string()),
                (self.settings.d21.clone(), (data.d21 * Self::get_measurement_factor(&self.settings.d21, current_factor, voltage_factor)).to_string()),
                (self.settings.d22.clone(), (data.d22 * Self::get_measurement_factor(&self.settings.d22, current_factor, voltage_factor)).to_string()),
                (self.settings.d23.clone(), (data.d23 * Self::get_measurement_factor(&self.settings.d23, current_factor, voltage_factor)).to_string()),
                (self.settings.d24.clone(), (data.d24 * Self::get_measurement_factor(&self.settings.d24, current_factor, voltage_factor)).to_string()),
                (self.settings.d25.clone(), (data.d25 * Self::get_measurement_factor(&self.settings.d25, current_factor, voltage_factor)).to_string()),
                (self.settings.d26.clone(), (data.d26 * Self::get_measurement_factor(&self.settings.d26, current_factor, voltage_factor)).to_string()),
                (self.settings.d27.clone(), (data.d27 * Self::get_measurement_factor(&self.settings.d27, current_factor, voltage_factor)).to_string()),
                (self.settings.d28.clone(), (data.d28 * Self::get_measurement_factor(&self.settings.d28, current_factor, voltage_factor)).to_string()),
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

    pub fn to_data_records(&self, event_id: u32, measurement_types: &[String], current_factor: f32, voltage_factor: f32) -> Vec<Vec<String>> {
        self.values_to_maps(event_id, current_factor, voltage_factor)
            .iter()
            .map(|data_map| {
                [
                    self.settings.meter.to_string(),
                    data_map.get("event_id").unwrap().to_string(),
                    data_map.get("timestamp").unwrap().to_string(),
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
