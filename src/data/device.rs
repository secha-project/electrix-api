use serde::Deserialize;
use crate::utils::tools::to_string;


#[derive(Clone, Debug, Deserialize)]
pub struct Device {
    pub id: u32,
    pub location: String,
    pub serialnumber: String,
    pub reading: Option<i32>,
    pub ik: f32,  // the current factor
    pub uk: f32,  // the voltage factor
}


impl Device {
    pub fn pretty_print(&self) -> String {
        let id: u32 = self.id;
        let serial_number: String = self.serialnumber.clone();
        let location: String = self.location.clone();
        let reading: String = to_string(self.reading.as_ref());
        let current_factor: f32 = self.ik;
        let voltage_factor: f32 = self.uk;

        "Device:\n".to_string() +
        &format!("- id: {id}\n") +
        &format!("- serial number: {serial_number}\n") +
        &format!("- location: {location}\n") +
        &format!("- reading: {reading}\n") +
        &format!("- current factor: {current_factor}\n") +
        &format!("- voltage factor: {voltage_factor}\n")
    }

    pub fn to_header_record() -> Vec<String> {
        vec![
            "id".to_string(),
            "serial number".to_string(),
            "location".to_string(),
            "reading".to_string(),
            "current factor".to_string(),
            "voltage factor".to_string()
        ]
    }

    pub fn to_record(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.serialnumber.clone(),
            self.location.clone(),
            to_string(self.reading.as_ref()),
            self.ik.to_string(),
            self.uk.to_string()
        ]
    }
}
