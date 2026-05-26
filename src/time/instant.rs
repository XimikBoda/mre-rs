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