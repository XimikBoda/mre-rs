#![allow(non_camel_case_types)]
use crate::mre_api;

pub type VM_TIMERPROC_T = extern "C" fn(tid: i32);

// Коди помилок MRE
pub const VM_TIMER_MTK_TIMER_NO_FREE: i32 = -5;
pub const VM_TIMER_ERROR_OF_PROCESS: i32 = -4;
pub const VM_TIMER_ERROR_OF_RES: i32 = -3;
pub const VM_TIMER_ERROR_OF_MEM: i32 = -2;
pub const VM_TIMER_ERROR: i32 = -1;
pub const VM_TIMER_OK: i32 = 0;

mre_api!(vm_create_timer(millisec: u32, timerproc: VM_TIMERPROC_T) -> i32 = -1);
mre_api!(vm_delete_timer(timerid: i32) -> i32 = -1);

mre_api!(vm_create_timer_ex(millisec: u32, timerproc: VM_TIMERPROC_T) -> i32 = -1);
mre_api!(vm_delete_timer_ex(timerid: i32) -> i32 = -1);