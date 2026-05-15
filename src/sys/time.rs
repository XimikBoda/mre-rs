#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct vm_time_t {
    pub year: i32,
    pub mon: i32,
    pub day: i32,
    pub hour: i32,
    pub min: i32,
    pub sec: i32,
}

mre_api!(vm_get_time(time: *mut vm_time_t) -> i32 = -1);
mre_api!(vm_get_curr_utc(utc: *mut u32) -> i32 = -1);
mre_api!(vm_get_utc(utc: *mut u32) -> i32 = -1);
mre_api!(vm_get_sys_time_zone() -> f32 = 0.0);
mre_api!(vm_get_tick_count() -> i32 = 0);