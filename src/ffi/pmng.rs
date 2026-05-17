#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::mre_api;
use core::ffi::{c_void};

pub type VM_P_HANDLE = i32;

pub const VM_PMNG_KERNEL_HANDLE: i32 = 0;
pub const VM_MAX_PATH: usize = 261;

pub const VM_PMNG_UNLOAD: i32 = 0;
pub const VM_PMNG_FOREGROUND: i32 = 1;
pub const VM_PMNG_BACKGROUND: i32 = 2;
pub const VM_PMNG_STARTING: i32 = 3;
pub const VM_PMNG_CLOSING: i32 = 4;
pub const VM_PMNG_INACTIVE: i32 = 5;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct vm_process_property {
    pub pHandle: VM_P_HANDLE,
    pub appID: i32,
    pub status: i32,
    pub pParentHandle: VM_P_HANDLE,
    pub process_type: i32,
    pub reserved: [i32; 5],
    pub filePath: [u16; VM_MAX_PATH],
}

mre_api!(vm_pmng_set_bg() -> i32 = -1);
mre_api!(vm_pmng_set_fg(file_name: *const u16) -> VM_P_HANDLE = -6);

mre_api!(vm_pmng_get_current_handle() -> VM_P_HANDLE = -1);
mre_api!(vm_pmng_get_handle(file_name: *const u16) -> VM_P_HANDLE = -1);

mre_api!(vm_pmng_get_process_list(handle_list: *mut VM_P_HANDLE, num: *mut u32) -> i32 = -1);
mre_api!(vm_pmng_get_process_property(handle: VM_P_HANDLE, property: *mut vm_process_property) -> i32 = -1);

mre_api!(vm_pmng_state(p_handle: VM_P_HANDLE) -> i32 = 0);

mre_api!(vm_start_app(filepath: *const u16, parent_app_handle: i32, is_parent_exit: i32));
mre_api!(vm_start_app_with_para(filepath: *const u16, parent_app_handle: i32, is_parent_exit: i32, parameter: *mut c_void, parameter_size: i32));
