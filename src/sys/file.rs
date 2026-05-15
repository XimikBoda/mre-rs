use core::ffi::{c_void};
use crate::mre_api;

pub const VM_FS_MODE_READ: u32 = 1;
pub const VM_FS_MODE_WRITE: u32 = 2;
pub const VM_FS_MODE_CREATE_ALWAYS_WRITE: u32 = 4;
pub const VM_FS_MODE_APPEND: u32 = 8;

pub const VM_FS_BASE_BEGIN: i32 = 1;
pub const VM_FS_BASE_CURR: i32 = 2;
pub const VM_FS_BASE_END: i32 = 3;

mre_api!(vm_file_open(filename: *const u16, mode: u32, binary: u32) -> i32 = -1);
mre_api!(vm_file_close(handle: i32));
mre_api!(vm_file_read(handle: i32, data: *mut c_void, length: u32, nread: *mut u32) -> i32 = -1);
mre_api!(vm_file_write(handle: i32, data: *mut c_void, length: u32, written: *mut u32) -> i32 = -1);
mre_api!(vm_file_commit(handle: i32) -> i32 = -1);
mre_api!(vm_file_seek(handle: i32, offset: i32, base: i32) -> i32 = -1);
mre_api!(vm_file_tell(handle: i32) -> i32 = -1);
mre_api!(vm_file_is_eof(handle: i32) -> i32 = -1);
mre_api!(vm_file_getfilesize(handle: i32, file_size: *mut u32) -> i32 = -1);