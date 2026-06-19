use crate::mre_api;

pub const MRE_SYS_SUBSCRIBER_ID: i32 = 1;
pub const MRE_SYS_EQUIPMENT_ID: i32 = 2;
pub const MRE_SYS_VERSION: i32 = 3;
pub const MRE_SYS_HOST_VERSION: i32 = 4;
pub const MRE_SYS_HOST_MAX_MEM: i32 = 5;
pub const MRE_SYS_HOME_DIR: i32 = 6;
pub const MRE_SYS_BUILD_DATE_TIME: i32 = 7;
pub const MRE_SYS_RELEASE_BRANCH: i32 = 8;

pub const VM_DEV_INFO_MODEL_NUMBER: i32 = 0;
pub const VM_DEV_INFO_FW_NUMBER: i32 = 1;
pub const VM_DEV_INFO_LANG_PACK: i32 = 2;

mre_api!(vm_get_sys_property(key: i32, value: *mut i8, len: u32) -> u32 = 0);
mre_api!(vm_get_origin_release_verno(value: *mut i8, len: u32) -> u32 = 0);
mre_api!(vm_get_device_info(info_type: i32, value: *mut i8) -> i32 = -1);

mre_api!(vm_get_current_lang_country_code() -> *mut u8 = core::ptr::null_mut());