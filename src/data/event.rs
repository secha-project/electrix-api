use serde::Deserialize;

use super::event_item::DeviceEventItemWithId;


#[derive(Clone, Debug, Deserialize)]
pub struct DeviceEvent {
    pub id: u32,
    pub meter: u32,
    pub triggertime: Option<String>,
    pub starttime: String,
    pub endtime: String,
}


impl DeviceEvent {
    pub fn pretty_print(&self) -> String {
        let id: u32 = self.id;
        let meter: u32 = self.meter;
        let triggertime: String = self.triggertime
            .as_ref()
            .map_or_else(|| "null".to_string(), std::clone::Clone::clone);
        let starttime: String = self.starttime.clone();
        let endtime: String = self.endtime.clone();

        "Device Event:\n".to_string() +
        &format!("- event id: {id}\n") +
        &format!("- meter: {meter}\n") +
        &format!("- trigger time: {triggertime}\n") +
        &format!("- start time: {starttime}\n") +
        &format!("- end time: {endtime}\n")
    }

    #[allow(clippy::similar_names, clippy::too_many_lines)]
    pub fn pretty_print_with_details(&self, details: &DeviceEventItemWithId) -> String {
        let id: u32 = self.id;
        let meter: u32 = self.meter;
        let triggertime: String = self.triggertime.as_ref().map_or_else(|| "null".to_string(), std::clone::Clone::clone);
        let starttime: String = self.starttime.clone();
        let endtime: String = self.endtime.clone();

        let settings_enabled: String = details.settings.enabled.clone();
        let settings_meter: u32 = details.settings.meter;
        let settings_timestamp: String = details.settings.timestamp.clone();
        let settings_pre_en_t: String = details.settings.pre_en_t.clone();
        let settings_post_en_t: String = details.settings.post_en_t.clone();
        let settings_pre_rec_t: String = details.settings.pre_rec_t.clone();
        let settings_post_rec_t: String = details.settings.post_rec_t.clone();
        let settings_a: String = details.settings.A.clone();
        let settings_b: String = details.settings.B.clone();
        let settings_c: String = details.settings.C.clone();
        let settings_d: String = details.settings.D.clone();
        let settings_e: String = details.settings.E.clone();
        let settings_f: String = details.settings.F.clone();
        let settings_g: String = details.settings.G.clone();
        let settings_h: String = details.settings.H.clone();
        let settings_i: String = details.settings.I.clone();
        let settings_logic: String = details.settings.logic.clone();
        let settings_d1: String = details.settings.d1.clone();
        let settings_d2: String = details.settings.d2.clone();
        let settings_d3: String = details.settings.d3.clone();
        let settings_d4: String = details.settings.d4.clone();
        let settings_d5: String = details.settings.d5.clone();
        let settings_d6: String = details.settings.d6.clone();
        let settings_d7: String = details.settings.d7.clone();
        let settings_d8: String = details.settings.d8.clone();
        let settings_d9: String = details.settings.d9.clone();
        let settings_d10: String = details.settings.d10.clone();
        let settings_d11: String = details.settings.d11.clone();
        let settings_d12: String = details.settings.d12.clone();
        let settings_d13: String = details.settings.d13.clone();
        let settings_d14: String = details.settings.d14.clone();
        let settings_d15: String = details.settings.d15.clone();
        let settings_d16: String = details.settings.d16.clone();
        let settings_d17: String = details.settings.d17.clone();
        let settings_d18: String = details.settings.d18.clone();
        let settings_d19: String = details.settings.d19.clone();
        let settings_d20: String = details.settings.d20.clone();
        let settings_d21: String = details.settings.d21.clone();
        let settings_d22: String = details.settings.d22.clone();
        let settings_d23: String = details.settings.d23.clone();
        let settings_d24: String = details.settings.d24.clone();
        let settings_d25: String = details.settings.d25.clone();
        let settings_d26: String = details.settings.d26.clone();
        let settings_d27: String = details.settings.d27.clone();
        let settings_d28: String = details.settings.d28.clone();

        let result: String = "Device Event:\n".to_string() +
        &format!("- event id: {id}\n") +
        &format!("- meter: {meter}\n") +
        &format!("- trigger time: {triggertime}\n") +
        &format!("- start time: {starttime}\n") +
        &format!("- end time: {endtime}\n") +
        "- settings:\n" +
        &format!("\t- enabled: {settings_enabled}\n") +
        &format!("\t- meter: {settings_meter}\n") +
        &format!("\t- timestamp: {settings_timestamp}\n") +
        &format!("\t- pre_en_t: {settings_pre_en_t}\n") +
        &format!("\t- post_en_t: {settings_post_en_t}\n") +
        &format!("\t- pre_rec_t: {settings_pre_rec_t}\n") +
        &format!("\t- post_rec_t: {settings_post_rec_t}\n") +
        &format!("\t- A: {settings_a}\n") +
        &format!("\t- B: {settings_b}\n") +
        &format!("\t- C: {settings_c}\n") +
        &format!("\t- D: {settings_d}\n") +
        &format!("\t- E: {settings_e}\n") +
        &format!("\t- F: {settings_f}\n") +
        &format!("\t- G: {settings_g}\n") +
        &format!("\t- H: {settings_h}\n") +
        &format!("\t- I: {settings_i}\n") +
        &format!("\t- logic: {settings_logic}\n");

        if let Some(event) = details.data.first() {
            let value_timestamp: String = event.timestamp.clone();
            let value_d1: f32 = event.d1;
            let value_d2: f32 = event.d2;
            let value_d3: f32 = event.d3;
            let value_d4: f32 = event.d4;
            let value_d5: f32 = event.d5;
            let value_d6: f32 = event.d6;
            let value_d7: f32 = event.d7;
            let value_d8: f32 = event.d8;
            let value_d9: f32 = event.d9;
            let value_d10: f32 = event.d10;
            let value_d11: f32 = event.d11;
            let value_d12: f32 = event.d12;
            let value_d13: f32 = event.d13;
            let value_d14: f32 = event.d14;
            let value_d15: f32 = event.d15;
            let value_d16: f32 = event.d16;
            let value_d17: f32 = event.d17;
            let value_d18: f32 = event.d18;
            let value_d19: f32 = event.d19;
            let value_d20: f32 = event.d20;
            let value_d21: f32 = event.d21;
            let value_d22: f32 = event.d22;
            let value_d23: f32 = event.d23;
            let value_d24: f32 = event.d24;
            let value_d25: f32 = event.d25;
            let value_d26: f32 = event.d26;
            let value_d27: f32 = event.d27;
            let value_d28: f32 = event.d28;

            return result + &format!("- total of {} data points\n", details.data.len())
            + "- first data point:\n" +
            &format!("\t- timestamp: {value_timestamp}\n") +
            &format!("\t- {settings_d1}: {value_d1}\n") +
            &format!("\t- {settings_d2}: {value_d2}\n") +
            &format!("\t- {settings_d3}: {value_d3}\n") +
            &format!("\t- {settings_d4}: {value_d4}\n") +
            &format!("\t- {settings_d5}: {value_d5}\n") +
            &format!("\t- {settings_d6}: {value_d6}\n") +
            &format!("\t- {settings_d7}: {value_d7}\n") +
            &format!("\t- {settings_d8}: {value_d8}\n") +
            &format!("\t- {settings_d9}: {value_d9}\n") +
            &format!("\t- {settings_d10}: {value_d10}\n") +
            &format!("\t- {settings_d11}: {value_d11}\n") +
            &format!("\t- {settings_d12}: {value_d12}\n") +
            &format!("\t- {settings_d13}: {value_d13}\n") +
            &format!("\t- {settings_d14}: {value_d14}\n") +
            &format!("\t- {settings_d15}: {value_d15}\n") +
            &format!("\t- {settings_d16}: {value_d16}\n") +
            &format!("\t- {settings_d17}: {value_d17}\n") +
            &format!("\t- {settings_d18}: {value_d18}\n") +
            &format!("\t- {settings_d19}: {value_d19}\n") +
            &format!("\t- {settings_d20}: {value_d20}\n") +
            &format!("\t- {settings_d21}: {value_d21}\n") +
            &format!("\t- {settings_d22}: {value_d22}\n") +
            &format!("\t- {settings_d23}: {value_d23}\n") +
            &format!("\t- {settings_d24}: {value_d24}\n") +
            &format!("\t- {settings_d25}: {value_d25}\n") +
            &format!("\t- {settings_d26}: {value_d26}\n") +
            &format!("\t- {settings_d27}: {value_d27}\n") +
            &format!("\t- {settings_d28}: {value_d28}\n")
        }

        result + "- no data points\n"
    }

    pub fn to_header_record() -> Vec<String> {
        vec![
            "event_id".to_string(),
            "meter".to_string(),
            "triggertime".to_string(),
            "starttime".to_string(),
            "endtime".to_string(),
        ]
    }

    pub fn to_record(&self) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.meter.to_string(),
            self.triggertime.clone().map_or_else(String::new, |time| time),
            self.starttime.clone(),
            self.endtime.clone(),
        ]
    }
}
