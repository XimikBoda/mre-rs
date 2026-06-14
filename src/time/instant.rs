use core::time::Duration;
use crate::ffi::time::*;

#[derive(Clone, Copy, Debug)]
pub struct Instant {
    pub ticks: u32,
}

impl Instant {
    pub fn now() -> Self {
        let ticks = unsafe { vm_get_tick_count() } as u32;
        Self { ticks }
    }

    pub fn elapsed(&self) -> Duration {
        let current = Self::now().ticks;
        let diff_ms = current.wrapping_sub(self.ticks);
        Duration::from_millis(diff_ms as u64)
    }
}

static mut BOOT_TIME_MS: u64 = 0;

#[unsafe(no_mangle)]
pub extern "C" fn external_current_millis() -> u64 {
    let ticks_ms = Instant::now().ticks as u64;

    unsafe {
        if BOOT_TIME_MS == 0 {
            let utc_sec = crate::time::datetime::utc_timestamp().unwrap_or(0);
            
            if utc_sec > 0 {
                BOOT_TIME_MS = (utc_sec * 1000).saturating_sub(ticks_ms);
            } else {
                return ticks_ms;
            }
        }
        BOOT_TIME_MS + ticks_ms
    }
}