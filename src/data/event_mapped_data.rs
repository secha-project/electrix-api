use serde::Deserialize;
use crate::utils::tools::{to_string, to_string_with_factor};

#[derive(Clone, Debug, Deserialize)]
pub struct DeviceEventMappedData {
    pub timestamp: String,
    #[serde(rename = "UL1")]
    pub ul1: Option<f32>,
    #[serde(rename = "UL2")]
    pub ul2: Option<f32>,
    #[serde(rename = "UL3")]
    pub ul3: Option<f32>,
    #[serde(rename = "U12")]
    pub u12: Option<f32>,
    #[serde(rename = "U23")]
    pub u23: Option<f32>,
    #[serde(rename = "U31")]
    pub u31: Option<f32>,
    #[serde(rename = "IL1")]
    pub il1: Option<f32>,
    #[serde(rename = "IL2")]
    pub il2: Option<f32>,
    #[serde(rename = "IL3")]
    pub il3: Option<f32>,
    #[serde(rename = "IN")]
    pub i_n: Option<f32>,
    #[serde(rename = "TDUL1")]
    pub tdu_l1: Option<f32>,
    #[serde(rename = "TDUL2")]
    pub tdu_l2: Option<f32>,
    #[serde(rename = "TDUL3")]
    pub tdu_l3: Option<f32>,
    #[serde(rename = "TDIL1")]
    pub tdi_l1: Option<f32>,
    #[serde(rename = "TDIL2")]
    pub tdi_l2: Option<f32>,
    #[serde(rename = "TDIL3")]
    pub tdi_l3: Option<f32>,
    #[serde(rename = "P1L1")]
    pub p1l1: Option<f32>,
    #[serde(rename = "P1L2")]
    pub p1l2: Option<f32>,
    #[serde(rename = "P1L3")]
    pub p1l3: Option<f32>,
    #[serde(rename = "Q1L1")]
    pub q1l1: Option<f32>,
    #[serde(rename = "Q1L2")]
    pub q1l2: Option<f32>,
    #[serde(rename = "Q1L3")]
    pub q1l3: Option<f32>,
    #[serde(rename = "DPFL1")]
    pub dpf_l1: Option<f32>,
    #[serde(rename = "DPFL2")]
    pub dpf_l2: Option<f32>,
    #[serde(rename = "DPFL3")]
    pub dpf_l3: Option<f32>,
    pub freq: Option<f32>,
    #[serde(rename = "U2/U1")]
    pub u2_to_u1_ratio: Option<f32>,
    #[serde(rename = "U0/U1")]
    pub u0_to_u1_ratio: Option<f32>,
}

impl DeviceEventMappedData {
    pub fn to_header_record() -> Vec<String> {
        vec![
            "timestamp".to_string(),
            "UL1".to_string(),
            "UL2".to_string(),
            "UL3".to_string(),
            "U12".to_string(),
            "U23".to_string(),
            "U31".to_string(),
            "IL1".to_string(),
            "IL2".to_string(),
            "IL3".to_string(),
            "IN".to_string(),
            "TDUL1".to_string(),
            "TDUL2".to_string(),
            "TDUL3".to_string(),
            "TDIL1".to_string(),
            "TDIL2".to_string(),
            "TDIL3".to_string(),
            "P1L1".to_string(),
            "P1L2".to_string(),
            "P1L3".to_string(),
            "Q1L1".to_string(),
            "Q1L2".to_string(),
            "Q1L3".to_string(),
            "DPFL1".to_string(),
            "DPFL2".to_string(),
            "DPFL3".to_string(),
            "freq".to_string(),
            "U2/U1".to_string(),
            "U0/U1".to_string(),
        ]
    }

    pub fn to_record(&self, current_factor: f32, voltage_factor: f32) -> Vec<String> {
        let power_factor = current_factor * voltage_factor;

        // the order of the fields must match the order in the header
        vec![
            format!("{}Z", self.timestamp.clone()),
            to_string_with_factor(self.ul1, voltage_factor),
            to_string_with_factor(self.ul2, voltage_factor),
            to_string_with_factor(self.ul3, voltage_factor),
            to_string_with_factor(self.u12, voltage_factor),
            to_string_with_factor(self.u23, voltage_factor),
            to_string_with_factor(self.u31, voltage_factor),
            to_string_with_factor(self.il1, current_factor),
            to_string_with_factor(self.il2, current_factor),
            to_string_with_factor(self.il3, current_factor),
            to_string_with_factor(self.i_n, current_factor),
            to_string_with_factor(self.tdu_l1, voltage_factor),
            to_string_with_factor(self.tdu_l2, voltage_factor),
            to_string_with_factor(self.tdu_l3, voltage_factor),
            to_string_with_factor(self.tdi_l1, current_factor),
            to_string_with_factor(self.tdi_l2, current_factor),
            to_string_with_factor(self.tdi_l3, current_factor),
            to_string_with_factor(self.p1l1, power_factor),
            to_string_with_factor(self.p1l2, power_factor),
            to_string_with_factor(self.p1l3, power_factor),
            to_string_with_factor(self.q1l1, power_factor),
            to_string_with_factor(self.q1l2, power_factor),
            to_string_with_factor(self.q1l3, power_factor),
            to_string(self.dpf_l1.as_ref()),
            to_string(self.dpf_l2.as_ref()),
            to_string(self.dpf_l3.as_ref()),
            to_string(self.freq.as_ref()),
            to_string(self.u2_to_u1_ratio.as_ref()),
            to_string(self.u0_to_u1_ratio.as_ref()),
        ]
    }
}
