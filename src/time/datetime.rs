use crate::ffi::time::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl From<vm_time_t> for DateTime {
    fn from(vt: vm_time_t) -> Self {
        Self {
            year: vt.year,
            month: vt.mon as u8,
            day: vt.day as u8,
            hour: vt.hour as u8,
            minute: vt.min as u8,
            second: vt.sec as u8,
        }
    }
}

impl Into<vm_time_t> for DateTime {
    fn into(self) -> vm_time_t {
        vm_time_t {
            year: self.year,
            mon: self.month as i32,
            day: self.day as i32,
            hour: self.hour as i32,
            min: self.minute as i32,
            sec: self.second as i32,
        }
    }
}

pub fn now() -> Result<DateTime, i32> {
    let mut vt: vm_time_t = unsafe { core::mem::zeroed() };
    let res = unsafe { vm_get_time(&mut vt) };
    
    if res < 0 {
        Err(res)
    } else {
        Ok(DateTime::from(vt))
    }
}

pub fn utc_timestamp() -> Result<u64, i32> {
    let mut utc: u32 = 0;
    let res = unsafe { vm_get_utc(&mut utc) };
    
    if res < 0 { Err(res) } else { Ok(utc as u64) }
}

pub fn timezone() -> f32 {
    unsafe { vm_get_sys_time_zone() }
}