use serde::Deserialize;


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
    "i1l3,i3l3,i5l3,i7l3,i9l3,i11l3,i13l3,i15l3,i17l3,i19l3,i21l3,i23l3,i25l3,i27l3,i29l3,i31l3,i33l3,i35l3,i37l3,i39l3,",
    "u2u1,u0u1,udcl1,udcl2,udcl3,idcl1,idcl2,idcl3,idcn,t1,t2,t3,t4,udcl12,udcl23,udcl31,pfl1,pfl2,pfl3,dpfl1,dpfl2,dpfl3,",
    "udevl1,odevl1,udevl2,odevl2,udevl3,odevl3,umaxl12,umaxl23,umaxl31,umaxl1v,umaxl2v,umaxl3v,imaxl1a,imaxl2a,imaxl3a,imaxn,",
    "dmax,tdimax,u2u1max,u0u1max,uminl1v,uminl2v,uminl3v,iminl1a,iminl2a,iminl3a,uminl12,uminl23,uminl31,pfmin,dpfmin,",
    "pstl1,pstl2,pstl3,jd,js,jus,rec_source,io,flagged,periods,samples,calc_t,m0peak_t,crc_status,",
    "io1,io2,io3,io4,io5,io6,io7,io8,ep10wh,ep_10wh,eq10varh,eq_10varh,eq1_pos,eq1_neg,rec_no,dl1,dl2,dl3",
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
    pub u2u1: f32,
    pub u0u1: f32,
    pub udcl1: f32,
    pub udcl2: f32,
    pub udcl3: f32,
    pub idcl1: f32,
    pub idcl2: f32,
    pub idcl3: f32,
    pub idcn: f32,
    pub t1: f32,
    pub t2: f32,
    pub t3: f32,
    pub t4: f32,
    pub udcl12: f32,
    pub udcl23: f32,
    pub udcl31: f32,
    pub pfl1: f32,
    pub pfl2: f32,
    pub pfl3: f32,
    pub dpfl1: f32,
    pub dpfl2: f32,
    pub dpfl3: f32,
    pub udevl1: f32,
    pub odevl1: f32,
    pub udevl2: f32,
    pub odevl2: f32,
    pub udevl3: f32,
    pub odevl3: f32,
    pub umaxl12: f32,
    pub umaxl23: f32,
    pub umaxl31: f32,
    pub umaxl1v: f32,
    pub umaxl2v: f32,
    pub umaxl3v: f32,
    pub imaxl1a: f32,
    pub imaxl2a: f32,
    pub imaxl3a: f32,
    pub imaxn: f32,
    pub dmax: f32,
    pub tdimax: f32,
    pub u2u1max: f32,
    pub u0u1max: f32,
    pub uminl1v: f32,
    pub uminl2v: f32,
    pub uminl3v: f32,
    pub iminl1a: f32,
    pub iminl2a: f32,
    pub iminl3a: f32,
    pub uminl12: f32,
    pub uminl23: f32,
    pub uminl31: f32,
    pub pfmin: f32,
    pub dpfmin: f32,
    pub pstl1: f32,
    pub pstl2: f32,
    pub pstl3: f32,
    pub jd: i32,
    pub js: i32,
    pub jus: i32,
    pub rec_source: String,
    pub io: String,
    pub flagged: String,
    pub periods: i32,
    pub samples: Option<String>,
    pub calc_t: i32,
    pub m0peak_t: i32,
    pub crc_status: String,
    pub io1: i32,
    pub io2: i32,
    pub io3: i32,
    pub io4: i32,
    pub io5: i32,
    pub io6: i32,
    pub io7: i32,
    pub io8: i32,
    pub ep10wh: f32,
    pub ep_10wh: f32,
    pub eq10varh: f32,
    pub eq_10varh: f32,
    pub eq1_pos: f32,
    pub eq1_neg: f32,
    pub rec_no: i32,
    pub dl1: f32,
    pub dl2: f32,
    pub dl3: f32,
}


impl DeviceData {
    #[allow(clippy::similar_names, clippy::too_many_lines)]
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
        let u1l2: f32 = self.u1l2;
        let u3l2: f32 = self.u3l2;
        let u5l2: f32 = self.u5l2;
        let u7l2: f32 = self.u7l2;
        let u9l2: f32 = self.u9l2;
        let u1l3: f32 = self.u1l3;
        let u3l3: f32 = self.u3l3;
        let u5l3: f32 = self.u5l3;
        let u7l3: f32 = self.u7l3;
        let u9l3: f32 = self.u9l3;
        let i1l1: f32 = self.i1l1;
        let i3l1: f32 = self.i3l1;
        let i5l1: f32 = self.i5l1;
        let i7l1: f32 = self.i7l1;
        let i9l1: f32 = self.i9l1;
        let i1l2: f32 = self.i1l2;
        let i3l2: f32 = self.i3l2;
        let i5l2: f32 = self.i5l2;
        let i7l2: f32 = self.i7l2;
        let i9l2: f32 = self.i9l2;
        let i1l3: f32 = self.i1l3;
        let i3l3: f32 = self.i3l3;
        let i5l3: f32 = self.i5l3;
        let i7l3: f32 = self.i7l3;
        let i9l3: f32 = self.i9l3;

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
        &format!("- u1l2: {u1l2}\n") +
        &format!("- u3l2: {u3l2}\n") +
        &format!("- u5l2: {u5l2}\n") +
        &format!("- u7l2: {u7l2}\n") +
        &format!("- u9l2: {u9l2}\n") +
        &format!("- u1l3: {u1l3}\n") +
        &format!("- u3l3: {u3l3}\n") +
        &format!("- u5l3: {u5l3}\n") +
        &format!("- u7l3: {u7l3}\n") +
        &format!("- u9l3: {u9l3}\n") +
        &format!("- i1l1: {i1l1}\n") +
        &format!("- i3l1: {i3l1}\n") +
        &format!("- i5l1: {i5l1}\n") +
        &format!("- i7l1: {i7l1}\n") +
        &format!("- i9l1: {i9l1}\n") +
        &format!("- i1l2: {i1l2}\n") +
        &format!("- i3l2: {i3l2}\n") +
        &format!("- i5l2: {i5l2}\n") +
        &format!("- i7l2: {i7l2}\n") +
        &format!("- i9l2: {i9l2}\n") +
        &format!("- i1l3: {i1l3}\n") +
        &format!("- i3l3: {i3l3}\n") +
        &format!("- i5l3: {i5l3}\n") +
        &format!("- i7l3: {i7l3}\n") +
        &format!("- i9l3: {i9l3}\n")
    }

    pub fn to_header_record() -> Vec<String> {
        DATA_POINTS
            .split(',')
            .map(String::from)
            .collect()
    }

    #[allow(clippy::too_many_lines)]
    pub fn to_record(&self, current_factor: f32, voltage_factor: f32) -> Vec<String> {
        let power_factor = current_factor * voltage_factor;

        vec![
            self.id.to_string(),
            self.meter.to_string(),
            format!("{}Z", self.timestamp.clone()),
            self.fhz.to_string(),
            (self.pw * power_factor).to_string(),
            (self.qfryzevar * power_factor).to_string(),
            (self.q1var * power_factor).to_string(),
            (self.pl1w * power_factor).to_string(),
            (self.pl2w * power_factor).to_string(),
            (self.pl3w * power_factor).to_string(),
            (self.p1l1 * power_factor).to_string(),
            (self.p1l2 * power_factor).to_string(),
            (self.p1l3 * power_factor).to_string(),
            (self.qfryzel1var * power_factor).to_string(),
            (self.qfryzel2var * power_factor).to_string(),
            (self.qfryzel3var * power_factor).to_string(),
            (self.q1l1 * power_factor).to_string(),
            (self.q1l2 * power_factor).to_string(),
            (self.q1l3 * power_factor).to_string(),
            (self.sva * power_factor).to_string(),
            (self.sl1va * power_factor).to_string(),
            (self.sl2va * power_factor).to_string(),
            (self.sl3va * power_factor).to_string(),
            (self.s1l1 * power_factor).to_string(),
            (self.s1l2 * power_factor).to_string(),
            (self.s1l3 * power_factor).to_string(),
            (self.ul1v * voltage_factor).to_string(),
            (self.ul2v * voltage_factor).to_string(),
            (self.ul3v * voltage_factor).to_string(),
            (self.ul12v * voltage_factor).to_string(),
            (self.ul23v * voltage_factor).to_string(),
            (self.ul31v * voltage_factor).to_string(),
            (self.il1a * current_factor).to_string(),
            (self.il2a * current_factor).to_string(),
            (self.il3a * current_factor).to_string(),
            (self.i_n * current_factor).to_string(),
            (self.tdil1 * voltage_factor).to_string(),
            (self.tdil2 * voltage_factor).to_string(),
            (self.tdil3 * voltage_factor).to_string(),
            (self.u1l1 * voltage_factor).to_string(),
            (self.u3l1 * voltage_factor).to_string(),
            (self.u5l1 * voltage_factor).to_string(),
            (self.u7l1 * voltage_factor).to_string(),
            (self.u9l1 * voltage_factor).to_string(),
            (self.u11l1 * voltage_factor).to_string(),
            (self.u13l1 * voltage_factor).to_string(),
            (self.u15l1 * voltage_factor).to_string(),
            (self.u17l1 * voltage_factor).to_string(),
            (self.u19l1 * voltage_factor).to_string(),
            (self.u21l1 * voltage_factor).to_string(),
            (self.u23l1 * voltage_factor).to_string(),
            (self.u25l1 * voltage_factor).to_string(),
            (self.u27l1 * voltage_factor).to_string(),
            (self.u29l1 * voltage_factor).to_string(),
            (self.u31l1 * voltage_factor).to_string(),
            (self.u33l1 * voltage_factor).to_string(),
            (self.u35l1 * voltage_factor).to_string(),
            (self.u37l1 * voltage_factor).to_string(),
            (self.u39l1 * voltage_factor).to_string(),
            (self.u1l2 * voltage_factor).to_string(),
            (self.u3l2 * voltage_factor).to_string(),
            (self.u5l2 * voltage_factor).to_string(),
            (self.u7l2 * voltage_factor).to_string(),
            (self.u9l2 * voltage_factor).to_string(),
            (self.u11l2 * voltage_factor).to_string(),
            (self.u13l2 * voltage_factor).to_string(),
            (self.u15l2 * voltage_factor).to_string(),
            (self.u17l2 * voltage_factor).to_string(),
            (self.u19l2 * voltage_factor).to_string(),
            (self.u21l2 * voltage_factor).to_string(),
            (self.u23l2 * voltage_factor).to_string(),
            (self.u25l2 * voltage_factor).to_string(),
            (self.u27l2 * voltage_factor).to_string(),
            (self.u29l2 * voltage_factor).to_string(),
            (self.u31l2 * voltage_factor).to_string(),
            (self.u33l2 * voltage_factor).to_string(),
            (self.u35l2 * voltage_factor).to_string(),
            (self.u37l2 * voltage_factor).to_string(),
            (self.u39l2 * voltage_factor).to_string(),
            (self.u1l3 * voltage_factor).to_string(),
            (self.u3l3 * voltage_factor).to_string(),
            (self.u5l3 * voltage_factor).to_string(),
            (self.u7l3 * voltage_factor).to_string(),
            (self.u9l3 * voltage_factor).to_string(),
            (self.u11l3 * voltage_factor).to_string(),
            (self.u13l3 * voltage_factor).to_string(),
            (self.u15l3 * voltage_factor).to_string(),
            (self.u17l3 * voltage_factor).to_string(),
            (self.u19l3 * voltage_factor).to_string(),
            (self.u21l3 * voltage_factor).to_string(),
            (self.u23l3 * voltage_factor).to_string(),
            (self.u25l3 * voltage_factor).to_string(),
            (self.u27l3 * voltage_factor).to_string(),
            (self.u29l3 * voltage_factor).to_string(),
            (self.u31l3 * voltage_factor).to_string(),
            (self.u33l3 * voltage_factor).to_string(),
            (self.u35l3 * voltage_factor).to_string(),
            (self.u37l3 * voltage_factor).to_string(),
            (self.u39l3 * voltage_factor).to_string(),
            (self.i1l1 * current_factor).to_string(),
            (self.i3l1 * current_factor).to_string(),
            (self.i5l1 * current_factor).to_string(),
            (self.i7l1 * current_factor).to_string(),
            (self.i9l1 * current_factor).to_string(),
            (self.i11l1 * current_factor).to_string(),
            (self.i13l1 * current_factor).to_string(),
            (self.i15l1 * current_factor).to_string(),
            (self.i17l1 * current_factor).to_string(),
            (self.i19l1 * current_factor).to_string(),
            (self.i21l1 * current_factor).to_string(),
            (self.i23l1 * current_factor).to_string(),
            (self.i25l1 * current_factor).to_string(),
            (self.i27l1 * current_factor).to_string(),
            (self.i29l1 * current_factor).to_string(),
            (self.i31l1 * current_factor).to_string(),
            (self.i33l1 * current_factor).to_string(),
            (self.i35l1 * current_factor).to_string(),
            (self.i37l1 * current_factor).to_string(),
            (self.i39l1 * current_factor).to_string(),
            (self.i1l2 * current_factor).to_string(),
            (self.i3l2 * current_factor).to_string(),
            (self.i5l2 * current_factor).to_string(),
            (self.i7l2 * current_factor).to_string(),
            (self.i9l2 * current_factor).to_string(),
            (self.i11l2 * current_factor).to_string(),
            (self.i13l2 * current_factor).to_string(),
            (self.i15l2 * current_factor).to_string(),
            (self.i17l2 * current_factor).to_string(),
            (self.i19l2 * current_factor).to_string(),
            (self.i21l2 * current_factor).to_string(),
            (self.i23l2 * current_factor).to_string(),
            (self.i25l2 * current_factor).to_string(),
            (self.i27l2 * current_factor).to_string(),
            (self.i29l2 * current_factor).to_string(),
            (self.i31l2 * current_factor).to_string(),
            (self.i33l2 * current_factor).to_string(),
            (self.i35l2 * current_factor).to_string(),
            (self.i37l2 * current_factor).to_string(),
            (self.i39l2 * current_factor).to_string(),
            (self.i1l3 * current_factor).to_string(),
            (self.i3l3 * current_factor).to_string(),
            (self.i5l3 * current_factor).to_string(),
            (self.i7l3 * current_factor).to_string(),
            (self.i9l3 * current_factor).to_string(),
            (self.i11l3 * current_factor).to_string(),
            (self.i13l3 * current_factor).to_string(),
            (self.i15l3 * current_factor).to_string(),
            (self.i17l3 * current_factor).to_string(),
            (self.i19l3 * current_factor).to_string(),
            (self.i21l3 * current_factor).to_string(),
            (self.i23l3 * current_factor).to_string(),
            (self.i25l3 * current_factor).to_string(),
            (self.i27l3 * current_factor).to_string(),
            (self.i29l3 * current_factor).to_string(),
            (self.i31l3 * current_factor).to_string(),
            (self.i33l3 * current_factor).to_string(),
            (self.i35l3 * current_factor).to_string(),
            (self.i37l3 * current_factor).to_string(),
            (self.i39l3 * current_factor).to_string(),
            (self.u2u1 * voltage_factor).to_string(),
            (self.u0u1 * voltage_factor).to_string(),
            (self.udcl1 * voltage_factor).to_string(),
            (self.udcl2 * voltage_factor).to_string(),
            (self.udcl3 * voltage_factor).to_string(),
            (self.idcl1 * current_factor).to_string(),
            (self.idcl2 * current_factor).to_string(),
            (self.idcl3 * current_factor).to_string(),
            (self.idcn * current_factor).to_string(),
            self.t1.to_string(),
            self.t2.to_string(),
            self.t3.to_string(),
            self.t4.to_string(),
            (self.udcl12 * voltage_factor).to_string(),
            (self.udcl23 * voltage_factor).to_string(),
            (self.udcl31 * voltage_factor).to_string(),
            (self.pfl1 * power_factor).to_string(),
            (self.pfl2 * power_factor).to_string(),
            (self.pfl3 * power_factor).to_string(),
            self.dpfl1.to_string(),
            self.dpfl2.to_string(),
            self.dpfl3.to_string(),
            (self.udevl1 * voltage_factor).to_string(),
            (self.odevl1 * voltage_factor).to_string(),
            (self.udevl2 * voltage_factor).to_string(),
            (self.odevl2 * voltage_factor).to_string(),
            (self.udevl3 * voltage_factor).to_string(),
            (self.odevl3 * voltage_factor).to_string(),
            (self.umaxl12 * voltage_factor).to_string(),
            (self.umaxl23 * voltage_factor).to_string(),
            (self.umaxl31 * voltage_factor).to_string(),
            (self.umaxl1v * voltage_factor).to_string(),
            (self.umaxl2v * voltage_factor).to_string(),
            (self.umaxl3v * voltage_factor).to_string(),
            (self.imaxl1a * current_factor).to_string(),
            (self.imaxl2a * current_factor).to_string(),
            (self.imaxl3a * current_factor).to_string(),
            (self.imaxn * current_factor).to_string(),
            self.dmax.to_string(),
            (self.tdimax * current_factor).to_string(),
            self.u2u1max.to_string(),
            self.u0u1max.to_string(),
            (self.uminl1v * voltage_factor).to_string(),
            (self.uminl2v * voltage_factor).to_string(),
            (self.uminl3v * voltage_factor).to_string(),
            (self.iminl1a * current_factor).to_string(),
            (self.iminl2a * current_factor).to_string(),
            (self.iminl3a * current_factor).to_string(),
            (self.uminl12 * voltage_factor).to_string(),
            (self.uminl23 * voltage_factor).to_string(),
            (self.uminl31 * voltage_factor).to_string(),
            (self.pfmin * power_factor).to_string(),
            (self.dpfmin * power_factor).to_string(),
            (self.pstl1 * power_factor).to_string(),
            (self.pstl2 * power_factor).to_string(),
            (self.pstl3 * power_factor).to_string(),
            self.jd.to_string(),
            self.js.to_string(),
            self.jus.to_string(),
            self.rec_source.clone(),
            self.io.clone(),
            self.flagged.clone(),
            self.periods.to_string(),
            self.samples.clone().map_or_else(String::new, |s| s),
            self.calc_t.to_string(),
            self.m0peak_t.to_string(),
            self.crc_status.clone(),
            self.io1.to_string(),
            self.io2.to_string(),
            self.io3.to_string(),
            self.io4.to_string(),
            self.io5.to_string(),
            self.io6.to_string(),
            self.io7.to_string(),
            self.io8.to_string(),
            (self.ep10wh * power_factor).to_string(),
            (self.ep_10wh * power_factor).to_string(),
            (self.eq10varh * power_factor).to_string(),
            (self.eq_10varh * power_factor).to_string(),
            (self.eq1_pos * power_factor).to_string(),
            (self.eq1_neg * power_factor).to_string(),
            self.rec_no.to_string(),
            self.dl1.to_string(),
            self.dl2.to_string(),
            self.dl3.to_string(),
        ]
    }
}
