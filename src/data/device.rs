use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct Device {
    pub id: u32,
    pub location: String,
    pub serialnumber: String,
    pub reading: i32,
    pub ik: i32,  // the current factor
    pub uk: i32,  // the voltage factor
}


impl Device {
    pub fn pretty_print(&self) -> String {
        let id: u32 = self.id;
        let serial_number: String = self.serialnumber.clone();
        let location: String = self.location.clone();
        let reading: i32 = self.reading;
        let current_factor: i32 = self.ik;
        let voltage_factor: i32 = self.uk;

        "Device:\n".to_string() +
        &format!("- id: {id}\n") +
        &format!("- serial number: {serial_number}\n") +
        &format!("- location: {location}\n") +
        &format!("- reading: {reading}\n") +
        &format!("- current factor: {current_factor}\n") +
        &format!("- voltage factor: {voltage_factor}\n")
    }
}
