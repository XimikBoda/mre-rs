#![allow(non_camel_case_types)]
use crate::mre_api;
use crate::sys::time::vm_time_t;

pub const MAX_APP_NAME_LEN: usize = 260;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct vm_fileinfo_ext {
    pub filefullname: [u16; MAX_APP_NAME_LEN],
    pub filename: [u8; 8],
    pub extension: [u8; 3],
    pub attributes: u8,
    pub create_datetime: vm_time_t,
    pub modify_datetime: vm_time_t,
    pub filesize: u32,
    pub drive: u32,
    pub stamp: u32,
}

mre_api!(vm_get_exec_filename(filename: *mut u16) -> i32 = -1);

mre_api!(vm_file_delete(filename: *const u16) -> i32 = -1);
mre_api!(vm_file_rename(oldname: *const u16, newname: *const u16) -> i32 = -1);
mre_api!(vm_file_mkdir(dirname: *const u16) -> i32 = -1);
mre_api!(vm_file_rmdir(dirname: *const u16) -> i32 = -1);

mre_api!(vm_file_set_attributes(filename: *const u16, attributes: u8) -> i32 = -1);
mre_api!(vm_file_get_attributes(filename: *const u16) -> i32 = -1);

mre_api!(vm_file_get_modify_time(filename: *const u16, modify_time: *mut vm_time_t) -> i32 = -1);

mre_api!(vm_find_first_ext(pathname: *const u16, direntry: *mut vm_fileinfo_ext) -> i32 = -1);
mre_api!(vm_find_next_ext(handle: i32, direntry: *mut vm_fileinfo_ext) -> i32 = -1);
mre_api!(vm_find_close_ext(handle: i32));

pub const VM_FS_MOVE_PGS_FAIL: i32 = -1;
pub const VM_FS_MOVE_PGS_PREPARE: i32 = 0;
pub const VM_FS_MOVE_PGS_START: i32 = 1;
pub const VM_FS_MOVE_PGS_ING: i32 = 2;
pub const VM_FS_MOVE_PGS_DONE: i32 = 3;

pub type vm_file_copy_cb = extern "C" fn(act: i32, total: u32, completed: u32, hdl: i32) -> i32;

mre_api!(vm_file_copy(dst: *const u16, src: *const u16, cb: vm_file_copy_cb) -> i32 = -1);
mre_api!(vm_file_copy_abort(hdl: i32) -> i32 = -1);