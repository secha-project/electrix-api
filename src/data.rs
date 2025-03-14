#[allow(non_snake_case, clippy::similar_names, clippy::too_many_lines)]
pub mod data_structs {
    use serde::Deserialize;

    #[derive(Clone, Debug)]
    pub struct Host {
        host_url: String,
        token_header: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct Device {
        pub id: u32,
        pub location: String,
        pub serialnumber: String,
        pub reading: i32,
        pub ik: i32,  // the current factor
        pub uk: i32,  // the voltage factor
    }

    pub const DATA_POINTS: &str = concat!(
        "id,meter,timestamp,fhz,pw,qfryzevar,q1var,",
        "pl1w,pl2w,pl3w,p1l1,p1l2,p1l3,",
        "qfryzel1var,qfryzel2var,qfryzel3var,q1l1,q1l2,q1l3,",
        "sva,sl1va,sl2va,sl3va,s1l1,s1l2,s1l3,",
        "ul1v,ul2v,ul3v,ul12v,ul23v,ul31v,",
        "il1a,il2a,il3a,i_n,",
        "tdil1,tdil2,tdil3,",
        "u1l1,u3l1,u5l1,u7l1,u9l1,u11l1,u13l1,u15l1,u17l1,u19l1,u21l1,u23l1,u25l1,u27l1,u29l1,u31l1,u33l1,u35l1,u37l1,u39l1,",
        "u1l2,u3l2,u5l2,u7l2,u9l2,u11l2,u13l2,u15l2,u17l2,u19l2,u21l2,u23l2,u25l2,u27l2,u29l2,u31l2,u33l2,u35l2,u37l2,u39l2,",
        "u1l3,u3l3,u5l3,u7l3,u9l3,u11l3,u13l3,u15l3,u17l3,u19l3,u21l3,u23l3,u25l3,u27l3,u29l3,u31l3,u33l3,u35l3,u37l3,u39l3,",
        "i1l1,i3l1,i5l1,i7l1,i9l1,i11l1,i13l1,i15l1,i17l1,i19l1,i21l1,i23l1,i25l1,i27l1,i29l1,i31l1,i33l1,i35l1,i37l1,i39l1,",
        "i1l2,i3l2,i5l2,i7l2,i9l2,i11l2,i13l2,i15l2,i17l2,i19l2,i21l2,i23l2,i25l2,i27l2,i29l2,i31l2,i33l2,i35l2,i37l2,i39l2,",
        "i1l3,i3l3,i5l3,i7l3,i9l3,i11l3,i13l3,i15l3,i17l3,i19l3,i21l3,i23l3,i25l3,i27l3,i29l3,i31l3,i33l3,i35l3,i37l3,i39l3",
    );


    #[derive(Clone, Debug, Deserialize)]
    pub struct DeviceData {
        pub id: u32,         // id for the measurements?
        pub meter: u32,      // id for the device
        pub timestamp: String,  // timestamp in ISO-8601 format, e.g., 2025-03-06T12:08:00
        pub fhz: f32,        // frequency
        pub pw: f32,         // total power
        pub qfryzevar: f32,
        pub q1var: f32,
        pub pl1w: f32,
        pub pl2w: f32,
        pub pl3w: f32,
        pub p1l1: f32,
        pub p1l2: f32,
        pub p1l3: f32,
        pub qfryzel1var: f32,
        pub qfryzel2var: f32,
        pub qfryzel3var: f32,
        pub q1l1: f32,
        pub q1l2: f32,
        pub q1l3: f32,
        pub sva: f32,
        pub sl1va: f32,
        pub sl2va: f32,
        pub sl3va: f32,
        pub s1l1: f32,
        pub s1l2: f32,
        pub s1l3: f32,
        pub ul1v: f32,
        pub ul2v: f32,
        pub ul3v: f32,
        pub ul12v: f32,
        pub ul23v: f32,
        pub ul31v: f32,
        pub il1a: f32,
        pub il2a: f32,
        pub il3a: f32,
        pub i_n: f32,
        pub tdil1: f32,
        pub tdil2: f32,
        pub tdil3: f32,
        pub u1l1: f32,
        pub u3l1: f32,
        pub u5l1: f32,
        pub u7l1: f32,
        pub u9l1: f32,
        pub u11l1: f32,
        pub u13l1: f32,
        pub u15l1: f32,
        pub u17l1: f32,
        pub u19l1: f32,
        pub u21l1: f32,
        pub u23l1: f32,
        pub u25l1: f32,
        pub u27l1: f32,
        pub u29l1: f32,
        pub u31l1: f32,
        pub u33l1: f32,
        pub u35l1: f32,
        pub u37l1: f32,
        pub u39l1: f32,
        pub u1l2: f32,
        pub u3l2: f32,
        pub u5l2: f32,
        pub u7l2: f32,
        pub u9l2: f32,
        pub u11l2: f32,
        pub u13l2: f32,
        pub u15l2: f32,
        pub u17l2: f32,
        pub u19l2: f32,
        pub u21l2: f32,
        pub u23l2: f32,
        pub u25l2: f32,
        pub u27l2: f32,
        pub u29l2: f32,
        pub u31l2: f32,
        pub u33l2: f32,
        pub u35l2: f32,
        pub u37l2: f32,
        pub u39l2: f32,
        pub u1l3: f32,
        pub u3l3: f32,
        pub u5l3: f32,
        pub u7l3: f32,
        pub u9l3: f32,
        pub u11l3: f32,
        pub u13l3: f32,
        pub u15l3: f32,
        pub u17l3: f32,
        pub u19l3: f32,
        pub u21l3: f32,
        pub u23l3: f32,
        pub u25l3: f32,
        pub u27l3: f32,
        pub u29l3: f32,
        pub u31l3: f32,
        pub u33l3: f32,
        pub u35l3: f32,
        pub u37l3: f32,
        pub u39l3: f32,
        pub i1l1: f32,
        pub i3l1: f32,
        pub i5l1: f32,
        pub i7l1: f32,
        pub i9l1: f32,
        pub i11l1: f32,
        pub i13l1: f32,
        pub i15l1: f32,
        pub i17l1: f32,
        pub i19l1: f32,
        pub i21l1: f32,
        pub i23l1: f32,
        pub i25l1: f32,
        pub i27l1: f32,
        pub i29l1: f32,
        pub i31l1: f32,
        pub i33l1: f32,
        pub i35l1: f32,
        pub i37l1: f32,
        pub i39l1: f32,
        pub i1l2: f32,
        pub i3l2: f32,
        pub i5l2: f32,
        pub i7l2: f32,
        pub i9l2: f32,
        pub i11l2: f32,
        pub i13l2: f32,
        pub i15l2: f32,
        pub i17l2: f32,
        pub i19l2: f32,
        pub i21l2: f32,
        pub i23l2: f32,
        pub i25l2: f32,
        pub i27l2: f32,
        pub i29l2: f32,
        pub i31l2: f32,
        pub i33l2: f32,
        pub i35l2: f32,
        pub i37l2: f32,
        pub i39l2: f32,
        pub i1l3: f32,
        pub i3l3: f32,
        pub i5l3: f32,
        pub i7l3: f32,
        pub i9l3: f32,
        pub i11l3: f32,
        pub i13l3: f32,
        pub i15l3: f32,
        pub i17l3: f32,
        pub i19l3: f32,
        pub i21l3: f32,
        pub i23l3: f32,
        pub i25l3: f32,
        pub i27l3: f32,
        pub i29l3: f32,
        pub i31l3: f32,
        pub i33l3: f32,
        pub i35l3: f32,
        pub i37l3: f32,
        pub i39l3: f32,
        // pub u2u1: f32,
        // pub u0u1: f32,
        // pub udcl1: f32,
        // pub udcl2: f32,
        // pub udcl3: f32,
        // pub idcl1: f32,
        // pub idcl2: f32,
        // pub idcl3: f32,
        // pub idcn: f32,
        // pub t1: f32,
        // pub t2: f32,
        // pub t3: f32,
        // pub t4: f32,
        // pub udcl12: f32,
        // pub udcl23: f32,
        // pub udcl31: f32,
        // pub pfl1: f32,
        // pub pfl2: f32,
        // pub pfl3: f32,
        // pub dpfl1: f32,
        // pub dpfl2: f32,
        // pub dpfl3: f32,
        // pub udevl1: f32,
        // pub odevl1: f32,
        // pub udevl2: f32,
        // pub odevl2: f32,
        // pub udevl3: f32,
        // pub odevl3: f32,
        // pub umaxl12: f32,
        // pub umaxl23: f32,
        // pub umaxl31: f32,
        // pub umaxl1v: f32,
        // pub umaxl2v: f32,
        // pub umaxl3v: f32,
        // pub imaxl1a: f32,
        // pub imaxl2a: f32,
        // pub imaxl3a: f32,
        // pub imaxn: f32,
        // pub dmax: f32,
        // pub tdimax: f32,
        // pub u2u1max: f32,
        // pub u0u1max: f32,
        // pub uminl1v: f32,
        // pub uminl2v: f32,
        // pub uminl3v: f32,
        // pub iminl1a: f32,
        // pub iminl2a: f32,
        // pub iminl3a: f32,
        // pub uminl12: f32,
        // pub uminl23: f32,
        // pub uminl31: f32,
        // pub pfmin: f32,
        // pub dpfmin: f32,
        // pub pstl1: f32,
        // pub pstl2: f32,
        // pub pstl3: f32,
        // pub jd: i32,
        // pub js: i32,
        // pub jus: i32,
        // pub rec_source: String,
        // pub io: String,
        // pub flagged: String,
        // pub periods: i32,
        // pub samples: String,
        // pub calc_t: i32,
        // pub m0peak_t: i32,
        // pub crc_status: String,
        // pub io1: i32,
        // pub io2: i32,
        // pub io3: i32,
        // pub io4: i32,
        // pub io5: i32,
        // pub io6: i32,
        // pub io7: i32,
        // pub io8: i32,
        // pub ep10wh: f32,
        // pub ep_10wh: f32,
        // pub eq10varh: f32,
        // pub eq_10varh: f32,
        // pub eq1_pos: f32,
        // pub eq1_neg: f32,
        // pub rec_no: i32,
        // pub dl1: f32,
        // pub dl2: f32,
        // pub dl3: f32,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct DeviceEvent {
        pub id: u32,
        pub meter: u32,
        pub triggertime: Option<String>,
        pub starttime: String,
        pub endtime: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct DeviceEventSettings {
        pub enabled: String,
        pub meter: u32,
        pub timestamp: String,
        pub pre_en_t: String,
        pub post_en_t: String,
        pub pre_rec_t: String,
        pub post_rec_t: String,
        pub A: String,
        pub B: String,
        pub C: String,
        pub D: String,
        pub E: String,
        pub F: String,
        pub G: String,
        pub H: String,
        pub I: String,
        pub logic: String,
        pub d1: String,
        pub d2: String,
        pub d3: String,
        pub d4: String,
        pub d5: String,
        pub d6: String,
        pub d7: String,
        pub d8: String,
        pub d9: String,
        pub d10: String,
        pub d11: String,
        pub d12: String,
        pub d13: String,
        pub d14: String,
        pub d15: String,
        pub d16: String,
        pub d17: String,
        pub d18: String,
        pub d19: String,
        pub d20: String,
        pub d21: String,
        pub d22: String,
        pub d23: String,
        pub d24: String,
        pub d25: String,
        pub d26: String,
        pub d27: String,
        pub d28: String,
    }

    #[derive(Clone, Debug, Deserialize)]
    pub struct DeviceEventData {
        pub timestamp: String,
        pub d1: f32,
        pub d2: f32,
        pub d3: f32,
        pub d4: f32,
        pub d5: f32,
        pub d6: f32,
        pub d7: f32,
        pub d8: f32,
        pub d9: f32,
        pub d10: f32,
        pub d11: f32,
        pub d12: f32,
        pub d13: f32,
        pub d14: f32,
        pub d15: f32,
        pub d16: f32,
        pub d17: f32,
        pub d18: f32,
        pub d19: f32,
        pub d20: f32,
        pub d21: f32,
        pub d22: f32,
        pub d23: f32,
        pub d24: f32,
        pub d25: f32,
        pub d26: f32,
        pub d27: f32,
        pub d28: f32,
}

    #[derive(Clone, Debug, Deserialize)]
    pub struct DeviceEventDetails {
        settings: DeviceEventSettings,
        data: Vec<DeviceEventData>,
    }


    impl Host {
        pub fn get_from_env() -> Result<Self, String> {
            const HOST_URL_STR: &str = "HOST_URL";
            const ACCESS_TOKEN_STR: &str = "ACCESS_TOKEN";
            const ERROR_MESSAGE: &str = "environment variable is not set";

            let host_url: String = std::env::var(HOST_URL_STR)
                .map_or_else(
                    |_| Err(format!("{HOST_URL_STR} {ERROR_MESSAGE}")),
                    Ok
                    // |url| Ok(url)
                )?;

            let token_header: String = std::env::var(ACCESS_TOKEN_STR)
                .map_or_else(
                    |_| Err(format!("{ACCESS_TOKEN_STR} {ERROR_MESSAGE}")),
                    |token| Ok(format!("Api-Key {token}"))
                )?;

            Ok(
                Self {
                    host_url,
                    token_header,
                }
            )
        }

        pub fn get_host_url(&self) -> String {
            self.host_url.clone()
        }

        pub fn get_headers(&self) -> Vec<(String, String)> {
            vec![
                ("Content-Type".to_string(), "application/json".to_string()),
                ("Authorization".to_string(), self.token_header.clone()),
            ]
        }
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


    impl DeviceData {
        pub fn pretty_print(&self) -> String {
            let id: u32 = self.id;
            let meter: u32 = self.meter;
            let timestamp: String = self.timestamp.clone();
            let fhz: f32 = self.fhz;
            let pw: f32 = self.pw;
            let qfryzevar: f32 = self.qfryzevar;
            let q1var: f32 = self.q1var;
            let pl1w: f32 = self.pl1w;
            let pl2w: f32 = self.pl2w;
            let pl3w: f32 = self.pl3w;
            let p1l1: f32 = self.p1l1;
            let p1l2: f32 = self.p1l2;
            let p1l3: f32 = self.p1l3;
            let qfryzel1var: f32 = self.qfryzel1var;
            let qfryzel2var: f32 = self.qfryzel2var;
            let qfryzel3var: f32 = self.qfryzel3var;
            let q1l1: f32 = self.q1l1;
            let q1l2: f32 = self.q1l2;
            let q1l3: f32 = self.q1l3;
            let sva: f32 = self.sva;
            let sl1va: f32 = self.sl1va;
            let sl2va: f32 = self.sl2va;
            let sl3va: f32 = self.sl3va;
            let s1l1: f32 = self.s1l1;
            let s1l2: f32 = self.s1l2;
            let s1l3: f32 = self.s1l3;
            let ul1v: f32 = self.ul1v;
            let ul2v: f32 = self.ul2v;
            let ul3v: f32 = self.ul3v;
            let ul12v: f32 = self.ul12v;
            let ul23v: f32 = self.ul23v;
            let ul31v: f32 = self.ul31v;
            let il1a: f32 = self.il1a;
            let il2a: f32 = self.il2a;
            let il3a: f32 = self.il3a;
            let i_n: f32 = self.i_n;
            let tdil1: f32 = self.tdil1;
            let tdil2: f32 = self.tdil2;
            let tdil3: f32 = self.tdil3;
            let u1l1: f32 = self.u1l1;
            let u3l1: f32 = self.u3l1;
            let u5l1: f32 = self.u5l1;
            let u7l1: f32 = self.u7l1;
            let u9l1: f32 = self.u9l1;
            let u11l1: f32 = self.u11l1;
            let u13l1: f32 = self.u13l1;
            let u15l1: f32 = self.u15l1;
            let u17l1: f32 = self.u17l1;
            let u19l1: f32 = self.u19l1;
            let u21l1: f32 = self.u21l1;
            let u23l1: f32 = self.u23l1;
            let u25l1: f32 = self.u25l1;
            let u27l1: f32 = self.u27l1;
            let u29l1: f32 = self.u29l1;
            let u31l1: f32 = self.u31l1;
            let u33l1: f32 = self.u33l1;
            let u35l1: f32 = self.u35l1;
            let u37l1: f32 = self.u37l1;
            let u39l1: f32 = self.u39l1;
            let u1l2: f32 = self.u1l2;
            let u3l2: f32 = self.u3l2;
            let u5l2: f32 = self.u5l2;
            let u7l2: f32 = self.u7l2;
            let u9l2: f32 = self.u9l2;
            let u11l2: f32 = self.u11l2;
            let u13l2: f32 = self.u13l2;
            let u15l2: f32 = self.u15l2;
            let u17l2: f32 = self.u17l2;
            let u19l2: f32 = self.u19l2;
            let u21l2: f32 = self.u21l2;
            let u23l2: f32 = self.u23l2;
            let u25l2: f32 = self.u25l2;
            let u27l2: f32 = self.u27l2;
            let u29l2: f32 = self.u29l2;
            let u31l2: f32 = self.u31l2;
            let u33l2: f32 = self.u33l2;
            let u35l2: f32 = self.u35l2;
            let u37l2: f32 = self.u37l2;
            let u39l2: f32 = self.u39l2;
            let u1l3: f32 = self.u1l3;
            let u3l3: f32 = self.u3l3;
            let u5l3: f32 = self.u5l3;
            let u7l3: f32 = self.u7l3;
            let u9l3: f32 = self.u9l3;
            let u11l3: f32 = self.u11l3;
            let u13l3: f32 = self.u13l3;
            let u15l3: f32 = self.u15l3;
            let u17l3: f32 = self.u17l3;
            let u19l3: f32 = self.u19l3;
            let u21l3: f32 = self.u21l3;
            let u23l3: f32 = self.u23l3;
            let u25l3: f32 = self.u25l3;
            let u27l3: f32 = self.u27l3;
            let u29l3: f32 = self.u29l3;
            let u31l3: f32 = self.u31l3;
            let u33l3: f32 = self.u33l3;
            let u35l3: f32 = self.u35l3;
            let u37l3: f32 = self.u37l3;
            let u39l3: f32 = self.u39l3;
            let i1l1: f32 = self.i1l1;
            let i3l1: f32 = self.i3l1;
            let i5l1: f32 = self.i5l1;
            let i7l1: f32 = self.i7l1;
            let i9l1: f32 = self.i9l1;
            let i11l1: f32 = self.i11l1;
            let i13l1: f32 = self.i13l1;
            let i15l1: f32 = self.i15l1;
            let i17l1: f32 = self.i17l1;
            let i19l1: f32 = self.i19l1;
            let i21l1: f32 = self.i21l1;
            let i23l1: f32 = self.i23l1;
            let i25l1: f32 = self.i25l1;
            let i27l1: f32 = self.i27l1;
            let i29l1: f32 = self.i29l1;
            let i31l1: f32 = self.i31l1;
            let i33l1: f32 = self.i33l1;
            let i35l1: f32 = self.i35l1;
            let i37l1: f32 = self.i37l1;
            let i39l1: f32 = self.i39l1;
            let i1l2: f32 = self.i1l2;
            let i3l2: f32 = self.i3l2;
            let i5l2: f32 = self.i5l2;
            let i7l2: f32 = self.i7l2;
            let i9l2: f32 = self.i9l2;
            let i11l2: f32 = self.i11l2;
            let i13l2: f32 = self.i13l2;
            let i15l2: f32 = self.i15l2;
            let i17l2: f32 = self.i17l2;
            let i19l2: f32 = self.i19l2;
            let i21l2: f32 = self.i21l2;
            let i23l2: f32 = self.i23l2;
            let i25l2: f32 = self.i25l2;
            let i27l2: f32 = self.i27l2;
            let i29l2: f32 = self.i29l2;
            let i31l2: f32 = self.i31l2;
            let i33l2: f32 = self.i33l2;
            let i35l2: f32 = self.i35l2;
            let i37l2: f32 = self.i37l2;
            let i39l2: f32 = self.i39l2;
            let i1l3: f32 = self.i1l3;
            let i3l3: f32 = self.i3l3;
            let i5l3: f32 = self.i5l3;
            let i7l3: f32 = self.i7l3;
            let i9l3: f32 = self.i9l3;
            let i11l3: f32 = self.i11l3;
            let i13l3: f32 = self.i13l3;
            let i15l3: f32 = self.i15l3;
            let i17l3: f32 = self.i17l3;
            let i19l3: f32 = self.i19l3;
            let i21l3: f32 = self.i21l3;
            let i23l3: f32 = self.i23l3;
            let i25l3: f32 = self.i25l3;
            let i27l3: f32 = self.i27l3;
            let i29l3: f32 = self.i29l3;
            let i31l3: f32 = self.i31l3;
            let i33l3: f32 = self.i33l3;
            let i35l3: f32 = self.i35l3;
            let i37l3: f32 = self.i37l3;
            let i39l3: f32 = self.i39l3;

            "Device Data:\n".to_string() +
            &format!("- id: {id}\n") +
            &format!("- meter: {meter}\n") +
            &format!("- timestamp: {timestamp}\n") +
            &format!("- fhz: {fhz}\n") +
            &format!("- pw: {pw}\n") +
            &format!("- qfryzevar: {qfryzevar}\n") +
            &format!("- q1var: {q1var}\n") +
            &format!("- pl1w: {pl1w}\n") +
            &format!("- pl2w: {pl2w}\n") +
            &format!("- pl3w: {pl3w}\n") +
            &format!("- p1l1: {p1l1}\n") +
            &format!("- p1l2: {p1l2}\n") +
            &format!("- p1l3: {p1l3}\n") +
            &format!("- qfryzel1var: {qfryzel1var}\n") +
            &format!("- qfryzel2var: {qfryzel2var}\n") +
            &format!("- qfryzel3var: {qfryzel3var}\n") +
            &format!("- q1l1: {q1l1}\n") +
            &format!("- q1l2: {q1l2}\n") +
            &format!("- q1l3: {q1l3}\n") +
            &format!("- sva: {sva}\n") +
            &format!("- sl1va: {sl1va}\n") +
            &format!("- sl2va: {sl2va}\n") +
            &format!("- sl3va: {sl3va}\n") +
            &format!("- s1l1: {s1l1}\n") +
            &format!("- s1l2: {s1l2}\n") +
            &format!("- s1l3: {s1l3}\n") +
            &format!("- ul1v: {ul1v}\n") +
            &format!("- ul2v: {ul2v}\n") +
            &format!("- ul3v: {ul3v}\n") +
            &format!("- ul12v: {ul12v}\n") +
            &format!("- ul23v: {ul23v}\n") +
            &format!("- ul31v: {ul31v}\n") +
            &format!("- il1a: {il1a}\n") +
            &format!("- il2a: {il2a}\n") +
            &format!("- il3a: {il3a}\n") +
            &format!("- i_n: {i_n}\n") +
            &format!("- tdil1: {tdil1}\n") +
            &format!("- tdil2: {tdil2}\n") +
            &format!("- tdil3: {tdil3}\n") +
            &format!("- u1l1: {u1l1}\n") +
            &format!("- u3l1: {u3l1}\n") +
            &format!("- u5l1: {u5l1}\n") +
            &format!("- u7l1: {u7l1}\n") +
            &format!("- u9l1: {u9l1}\n") +
            &format!("- u11l1: {u11l1}\n") +
            &format!("- u13l1: {u13l1}\n") +
            &format!("- u15l1: {u15l1}\n") +
            &format!("- u17l1: {u17l1}\n") +
            &format!("- u19l1: {u19l1}\n") +
            &format!("- u21l1: {u21l1}\n") +
            &format!("- u23l1: {u23l1}\n") +
            &format!("- u25l1: {u25l1}\n") +
            &format!("- u27l1: {u27l1}\n") +
            &format!("- u29l1: {u29l1}\n") +
            &format!("- u31l1: {u31l1}\n") +
            &format!("- u33l1: {u33l1}\n") +
            &format!("- u35l1: {u35l1}\n") +
            &format!("- u37l1: {u37l1}\n") +
            &format!("- u39l1: {u39l1}\n") +
            &format!("- u1l2: {u1l2}\n") +
            &format!("- u3l2: {u3l2}\n") +
            &format!("- u5l2: {u5l2}\n") +
            &format!("- u7l2: {u7l2}\n") +
            &format!("- u9l2: {u9l2}\n") +
            &format!("- u11l2: {u11l2}\n") +
            &format!("- u13l2: {u13l2}\n") +
            &format!("- u15l2: {u15l2}\n") +
            &format!("- u17l2: {u17l2}\n") +
            &format!("- u19l2: {u19l2}\n") +
            &format!("- u21l2: {u21l2}\n") +
            &format!("- u23l2: {u23l2}\n") +
            &format!("- u25l2: {u25l2}\n") +
            &format!("- u27l2: {u27l2}\n") +
            &format!("- u29l2: {u29l2}\n") +
            &format!("- u31l2: {u31l2}\n") +
            &format!("- u33l2: {u33l2}\n") +
            &format!("- u35l2: {u35l2}\n") +
            &format!("- u37l2: {u37l2}\n") +
            &format!("- u39l2: {u39l2}\n") +
            &format!("- u1l3: {u1l3}\n") +
            &format!("- u3l3: {u3l3}\n") +
            &format!("- u5l3: {u5l3}\n") +
            &format!("- u7l3: {u7l3}\n") +
            &format!("- u9l3: {u9l3}\n") +
            &format!("- u11l3: {u11l3}\n") +
            &format!("- u13l3: {u13l3}\n") +
            &format!("- u15l3: {u15l3}\n") +
            &format!("- u17l3: {u17l3}\n") +
            &format!("- u19l3: {u19l3}\n") +
            &format!("- u21l3: {u21l3}\n") +
            &format!("- u23l3: {u23l3}\n") +
            &format!("- u25l3: {u25l3}\n") +
            &format!("- u27l3: {u27l3}\n") +
            &format!("- u29l3: {u29l3}\n") +
            &format!("- u31l3: {u31l3}\n") +
            &format!("- u33l3: {u33l3}\n") +
            &format!("- u35l3: {u35l3}\n") +
            &format!("- u37l3: {u37l3}\n") +
            &format!("- u39l3: {u39l3}\n") +
            &format!("- i1l1: {i1l1}\n") +
            &format!("- i3l1: {i3l1}\n") +
            &format!("- i5l1: {i5l1}\n") +
            &format!("- i7l1: {i7l1}\n") +
            &format!("- i9l1: {i9l1}\n") +
            &format!("- i11l1: {i11l1}\n") +
            &format!("- i13l1: {i13l1}\n") +
            &format!("- i15l1: {i15l1}\n") +
            &format!("- i17l1: {i17l1}\n") +
            &format!("- i19l1: {i19l1}\n") +
            &format!("- i21l1: {i21l1}\n") +
            &format!("- i23l1: {i23l1}\n") +
            &format!("- i25l1: {i25l1}\n") +
            &format!("- i27l1: {i27l1}\n") +
            &format!("- i29l1: {i29l1}\n") +
            &format!("- i31l1: {i31l1}\n") +
            &format!("- i33l1: {i33l1}\n") +
            &format!("- i35l1: {i35l1}\n") +
            &format!("- i37l1: {i37l1}\n") +
            &format!("- i39l1: {i39l1}\n") +
            &format!("- i1l2: {i1l2}\n") +
            &format!("- i3l2: {i3l2}\n") +
            &format!("- i5l2: {i5l2}\n") +
            &format!("- i7l2: {i7l2}\n") +
            &format!("- i9l2: {i9l2}\n") +
            &format!("- i11l2: {i11l2}\n") +
            &format!("- i13l2: {i13l2}\n") +
            &format!("- i15l2: {i15l2}\n") +
            &format!("- i17l2: {i17l2}\n") +
            &format!("- i19l2: {i19l2}\n") +
            &format!("- i21l2: {i21l2}\n") +
            &format!("- i23l2: {i23l2}\n") +
            &format!("- i25l2: {i25l2}\n") +
            &format!("- i27l2: {i27l2}\n") +
            &format!("- i29l2: {i29l2}\n") +
            &format!("- i31l2: {i31l2}\n") +
            &format!("- i33l2: {i33l2}\n") +
            &format!("- i35l2: {i35l2}\n") +
            &format!("- i37l2: {i37l2}\n") +
            &format!("- i39l2: {i39l2}\n") +
            &format!("- i1l3: {i1l3}\n") +
            &format!("- i3l3: {i3l3}\n") +
            &format!("- i5l3: {i5l3}\n") +
            &format!("- i7l3: {i7l3}\n") +
            &format!("- i9l3: {i9l3}\n") +
            &format!("- i11l3: {i11l3}\n") +
            &format!("- i13l3: {i13l3}\n") +
            &format!("- i15l3: {i15l3}\n") +
            &format!("- i17l3: {i17l3}\n") +
            &format!("- i19l3: {i19l3}\n") +
            &format!("- i21l3: {i21l3}\n") +
            &format!("- i23l3: {i23l3}\n") +
            &format!("- i25l3: {i25l3}\n") +
            &format!("- i27l3: {i27l3}\n") +
            &format!("- i29l3: {i29l3}\n") +
            &format!("- i31l3: {i31l3}\n") +
            &format!("- i33l3: {i33l3}\n") +
            &format!("- i35l3: {i35l3}\n") +
            &format!("- i37l3: {i37l3}\n") +
            &format!("- i39l3: {i39l3}\n")
        }
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
            &format!("- id: {id}\n") +
            &format!("- meter: {meter}\n") +
            &format!("- trigger time: {triggertime}\n") +
            &format!("- start time: {starttime}\n") +
            &format!("- end time: {endtime}\n")
        }

        pub fn pretty_print_with_details(&self, details: &DeviceEventDetails) -> String {
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
            &format!("- id: {id}\n") +
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

                return result + "- first data point:\n" +
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
    }
}
