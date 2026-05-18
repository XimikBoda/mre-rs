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