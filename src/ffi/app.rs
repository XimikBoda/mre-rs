#![allow(non_camel_case_types)]
use crate::mre_api;

pub const VM_MSG_PAINT: i32 = 1;
pub const VM_MSG_ACTIVE: i32 = 2;
pub const VM_MSG_INACTIVE: i32 = 3;
pub const VM_MSG_CREATE: i32 = 4;
pub const VM_MSG_QUIT: i32 = 5;
pub const VM_MSG_HIDE: i32 = 6;
pub const VM_MSG_CARD_PLUG_OUT: i32 = 7;
pub const VM_MSG_SCREEN_ROTATE: i32 = 8;
pub const VM_MSG_PUSH: i32 = 9;

pub type vm_sysevt_cb = extern "C" fn(message: i32, param: i32);

mre_api!(vm_reg_sysevt_callback(f: vm_sysevt_cb));
mre_api!(vm_exit_app());
mre_api!(vm_graphic_flush_screen());
mre_api!(vm_pmng_set_bg() -> i32 = -1);
mre_api!(vm_pmng_set_fg(file_name: *const u16) -> i32 = -1);

pub const VM_PMNG_UNLOAD: i32 = 0;
pub const VM_PMNG_FOREGROUND: i32 = 1;
pub const VM_PMNG_BACKGROUND: i32 = 2;
pub const VM_PMNG_STARTING: i32 = 3;
pub const VM_PMNG_CLOSING: i32 = 4;
pub const VM_PMNG_INACTIVE: i32 = 5;

mre_api!(vm_pmng_get_current_handle() -> i32 = -1);
mre_api!(vm_pmng_state(p_handle: i32) -> i32 = 0);