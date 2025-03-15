use serde::Deserialize;

use super::{event_data::DeviceEventData, event_settings::DeviceEventSettings};


#[derive(Clone, Debug, Deserialize)]
pub struct DeviceEventItem {
    pub settings: DeviceEventSettings,
    pub data: Vec<DeviceEventData>,
}
