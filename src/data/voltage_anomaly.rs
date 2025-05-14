use serde::Deserialize;


#[derive(Clone, Debug, Deserialize)]
pub struct VoltageAnomaly {
    pub id: u32,
    pub meter: u32,
    pub timestamp: String,
    pub phase: i8,
    pub duration_ms: i32,
    pub upeakl1: f32,
    pub upeakl2: f32,
    pub upeakl3: f32,
    pub urmsl1: f32,
    pub urmsl2: f32,
    pub urmsl3: f32,
    pub il1a: f32,
    pub il2a: f32,
    pub il3a: f32,
}


impl VoltageAnomaly {
    pub fn pretty_print(&self) -> String {
        let id: u32 = self.id;
        let meter: u32 = self.meter;
        let timestamp: String = self.timestamp.clone();
        let phase: i8 = self.phase;
        let duration_ms: i32 = self.duration_ms;
        let upeakl1: f32 = self.upeakl1;
        let upeakl2: f32 = self.upeakl2;
        let upeakl3: f32 = self.upeakl3;
        let urmsl1: f32 = self.urmsl1;
        let urmsl2: f32 = self.urmsl2;
        let urmsl3: f32 = self.urmsl3;
        let il1a: f32 = self.il1a;
        let il2a: f32 = self.il2a;
        let il3a: f32 = self.il3a;

        "Voltage Anomaly:\n".to_string() +
        &format!("- id: {id}\n") +
        &format!("- meter: {meter}\n") +
        &format!("- timestamp: {timestamp}\n") +
        &format!("- phase: {phase}\n") +
        &format!("- duration (ms): {duration_ms}\n") +
        &format!("- U peak L1 (V): {upeakl1}\n") +
        &format!("- U peak L2 (V): {upeakl2}\n") +
        &format!("- U peak L3 (V): {upeakl3}\n") +
        &format!("- U RMS L1 (V): {urmsl1}\n") +
        &format!("- U RMS L2 (V): {urmsl2}\n") +
        &format!("- U RMS L3 (V): {urmsl3}\n") +
        &format!("- I L1 A (A): {il1a}\n") +
        &format!("- I L2 A (A): {il2a}\n") +
        &format!("- I L3 A (A): {il3a}\n")
    }

    pub fn to_header_record() -> Vec<String> {
        vec![
            "id".to_string(),
            "meter".to_string(),
            "timestamp".to_string(),
            "phase".to_string(),
            "duration_ms".to_string(),
            "upeakl1".to_string(),
            "upeakl2".to_string(),
            "upeakl3".to_string(),
            "urmsl1".to_string(),
            "urmsl2".to_string(),
            "urmsl3".to_string(),
            "il1a".to_string(),
            "il2a".to_string(),
            "il3a".to_string(),
        ]
    }

    pub fn to_record(&self, current_factor: f32) -> Vec<String> {
        vec![
            self.id.to_string(),
            self.meter.to_string(),
            format!("{}Z", self.timestamp.clone()),
            self.phase.to_string(),
            self.duration_ms.to_string(),
            self.upeakl1.to_string(),
            self.upeakl2.to_string(),
            self.upeakl3.to_string(),
            self.urmsl1.to_string(),
            self.urmsl2.to_string(),
            self.urmsl3.to_string(),
            (self.il1a * current_factor).to_string(),
            (self.il2a * current_factor).to_string(),
            (self.il3a * current_factor).to_string(),
        ]
    }
}
