#[allow(clippy::similar_names)]
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
        // pub tdil1: f32,
        // pub tdil2: f32,
        // pub tdil3: f32,
        // pub u1l1: f32,
        // pub u3l1: f32,
        // pub u5l1: f32,
        // pub u7l1: f32,
        // pub u9l1: f32,
        // pub u11l1: f32,
        // pub u13l1: f32,
        // pub u15l1: f32,
        // pub u17l1: f32,
        // pub u19l1: f32,
        // pub u21l1: f32,
        // pub u23l1: f32,
        // pub u25l1: f32,
        // pub u27l1: f32,
        // pub u29l1: f32,
        // pub u31l1: f32,
        // pub u33l1: f32,
        // pub u35l1: f32,
        // pub u37l1: f32,
        // pub u39l1: f32,
        // pub u1l2: f32,
        // pub u3l2: f32,
        // pub u5l2: f32,
        // pub u7l2: f32,
        // pub u9l2: f32,
        // pub u11l2: f32,
        // pub u13l2: f32,
        // pub u15l2: f32,
        // pub u17l2: f32,
        // pub u19l2: f32,
        // pub u21l2: f32,
        // pub u23l2: f32,
        // pub u25l2: f32,
        // pub u27l2: f32,
        // pub u29l2: f32,
        // pub u31l2: f32,
        // pub u33l2: f32,
        // pub u35l2: f32,
        // pub u37l2: f32,
        // pub u39l2: f32,
        // pub u1l3: f32,
        // pub u3l3: f32,
        // pub u5l3: f32,
        // pub u7l3: f32,
        // pub u9l3: f32,
        // pub u11l3: f32,
        // pub u13l3: f32,
        // pub u15l3: f32,
        // pub u17l3: f32,
        // pub u19l3: f32,
        // pub u21l3: f32,
        // pub u23l3: f32,
        // pub u25l3: f32,
        // pub u27l3: f32,
        // pub u29l3: f32,
        // pub u31l3: f32,
        // pub u33l3: f32,
        // pub u35l3: f32,
        // pub u37l3: f32,
        // pub u39l3: f32,
        // pub i1l1: f32,
        // pub i3l1: f32,
        // pub i5l1: f32,
        // pub i7l1: f32,
        // pub i9l1: f32,
        // pub i11l1: f32,
        // pub i13l1: f32,
        // pub i15l1: f32,
        // pub i17l1: f32,
        // pub i19l1: f32,
        // pub i21l1: f32,
        // pub i23l1: f32,
        // pub i25l1: f32,
        // pub i27l1: f32,
        // pub i29l1: f32,
        // pub i31l1: f32,
        // pub i33l1: f32,
        // pub i35l1: f32,
        // pub i37l1: f32,
        // pub i39l1: f32,
        // pub i1l2: f32,
        // pub i3l2: f32,
        // pub i5l2: f32,
        // pub i7l2: f32,
        // pub i9l2: f32,
        // pub i11l2: f32,
        // pub i13l2: f32,
        // pub i15l2: f32,
        // pub i17l2: f32,
        // pub i19l2: f32,
        // pub i21l2: f32,
        // pub i23l2: f32,
        // pub i25l2: f32,
        // pub i27l2: f32,
        // pub i29l2: f32,
        // pub i31l2: f32,
        // pub i33l2: f32,
        // pub i35l2: f32,
        // pub i37l2: f32,
        // pub i39l2: f32,
        // pub i1l3: f32,
        // pub i3l3: f32,
        // pub i5l3: f32,
        // pub i7l3: f32,
        // pub i9l3: f32,
        // pub i11l3: f32,
        // pub i13l3: f32,
        // pub i15l3: f32,
        // pub i17l3: f32,
        // pub i19l3: f32,
        // pub i21l3: f32,
        // pub i23l3: f32,
        // pub i25l3: f32,
        // pub i27l3: f32,
        // pub i29l3: f32,
        // pub i31l3: f32,
        // pub i33l3: f32,
        // pub i35l3: f32,
        // pub i37l3: f32,
        // pub i39l3: f32,
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
            &format!("- i_n: {i_n}\n")
        }
    }

}
