use core::ffi::c_void;

pub type GetSymEntryFunc = extern "cdecl" fn(*const u8) -> *mut c_void;

#[doc(hidden)]
pub static mut SYSTEM_GET_SYM_ENTRY: Option<GetSymEntryFunc> = None;