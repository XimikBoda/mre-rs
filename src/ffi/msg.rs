#![allow(non_camel_case_types)]
use crate::mre_api;
use crate::ffi::pmng::VM_P_HANDLE;

pub const VM_MESSAGE_ID_BASE: u32 = 1000;

pub type VM_MESSAGE_PROC = extern "C" fn(
    sender: VM_P_HANDLE, 
    msg_id: u32, 
    wparam: i32, 
    lparam: i32
) -> i32;

mre_api!(vm_reg_msg_proc(proc: VM_MESSAGE_PROC));

mre_api!(vm_send_msg(phandle: VM_P_HANDLE, msg_id: u32, wparam: i32, lparam: i32) -> i32 = -1);
mre_api!(vm_post_msg(phandle: VM_P_HANDLE, msg_id: u32, wparam: i32, lparam: i32) -> i32 = -1);

mre_api!(vm_appcomm_dispatch_msg());