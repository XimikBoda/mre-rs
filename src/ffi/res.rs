use crate::mre_api;
use core::ffi::{c_void};

#[repr(C)]
pub struct VmResReadHint {
    pub res_offset: i32,
    pub res_size: i32,
}

mre_api!(vm_load_resource(res_name: *const u8, res_size: *mut i32) -> *mut u8);
mre_api!(vm_load_resource_from_file(filename: *const u16, res_name: *const u8, res_size: *mut i32) -> *mut u8);

mre_api!(vm_resource_get_data(data: *mut u8, offset: u32, size: u32) -> i32 = -1);
mre_api!(vm_resource_get_data_from_file(filename: *const u16, data: *mut u8, offset: u32, size: u32) -> i32 = -1);

mre_api!(vm_get_resource_offset(res_name: *const u8) -> u32);
mre_api!(vm_get_resource_offset_from_file(filename: *const u16, res_name: *const u8) -> u32);

mre_api!(vm_load_resource_use_outside_memory(filename: *const u16, res_name: *const u8, buffer: *mut c_void, buffer_size: i32, hint: *mut VmResReadHint) -> i32);