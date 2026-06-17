extern crate alloc;
use alloc::string::{String, ToString};
use crate::ffi::sys::*;

pub enum SysProperty {
    SubscriberId = MRE_SYS_SUBSCRIBER_ID as isize,
    EquipmentId = MRE_SYS_EQUIPMENT_ID as isize,
    MreVersion = MRE_SYS_VERSION as isize,
    HostVersion = MRE_SYS_HOST_VERSION as isize,
    MaxMemory = MRE_SYS_HOST_MAX_MEM as isize,
    HomeDir = MRE_SYS_HOME_DIR as isize,
    BuildDateTime = MRE_SYS_BUILD_DATE_TIME as isize,
    ReleaseBranch = MRE_SYS_RELEASE_BRANCH as isize,
}

pub fn get_sys_property(prop: SysProperty) -> Option<String> {
    let mut buffer = [0u8; 64]; 
    
    let written_len = unsafe {
        vm_get_sys_property(prop as i32, buffer.as_mut_ptr() as *mut i8, buffer.len() as u32)
    };

    if written_len == 0 {
        return None;
    }

    let valid_bytes = &buffer[..written_len as usize];

    let clean_bytes = valid_bytes.split(|&b| b == 0).next().unwrap_or(valid_bytes);

    String::from_utf8(clean_bytes.to_vec()).ok()
}

pub fn get_origin_release_verno() -> Option<String> {
    let mut buffer = [0u8; 64]; 
    
    let written_len = unsafe {
        vm_get_origin_release_verno(buffer.as_mut_ptr() as *mut i8, buffer.len() as u32)
    };

    if written_len == 0 {
        return None;
    }

    let valid_bytes = &buffer[..written_len as usize];

    let clean_bytes = valid_bytes.split(|&b| b == 0).next().unwrap_or(valid_bytes);

    String::from_utf8(clean_bytes.to_vec()).ok()
}

pub enum DevInfo {
    ModelNumber = VM_DEV_INFO_MODEL_NUMBER as isize,
    FirmwareNumber = VM_DEV_INFO_FW_NUMBER as isize,
    LanguagePack = VM_DEV_INFO_LANG_PACK as isize,
}

pub fn get_dev_info(info: DevInfo) -> Option<String> {
    let mut buffer = [0u8; 256]; 
    
    let result = unsafe {
        vm_get_device_info(info as i32, buffer.as_mut_ptr() as *mut i8)
    };

    if result != 0 {
        return None;
    }

    let clean_bytes = buffer.split(|&b| b == 0).next().unwrap_or(&buffer);
    
    if clean_bytes.is_empty() {
        return None;
    }

    String::from_utf8(clean_bytes.to_vec()).ok()
}

pub fn get_country_code() -> Option<String> {
    unsafe {
        let ptr = vm_get_current_lang_country_code();
        
        if ptr.is_null() {
            return None;
        }

        let mut len = 0;
        while *ptr.add(len) != 0 {
            len += 1;
        }

        if len == 0 {
            return None;
        }

        let slice = core::slice::from_raw_parts(ptr, len);

        String::from_utf8(slice.to_vec()).ok()
    }
}

pub fn get_device_model() -> String {
    get_dev_info(DevInfo::ModelNumber)
        .or_else(|| get_sys_property(SysProperty::EquipmentId))
        .map(|s| s.trim_matches(char::from(0)).trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_system_version() -> String {
    get_dev_info(DevInfo::FirmwareNumber)
        .or_else(|| get_sys_property(SysProperty::ReleaseBranch))
        .map(|s| s.trim_matches(char::from(0)).trim().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

pub fn get_system_locale() -> String {
    get_country_code()
        .map(|s| s.trim_matches(char::from(0)).trim().to_string())
        .filter(|s| !s.is_empty()) 
        .unwrap_or_else(|| "en-US".to_string())
}

pub fn get_system_lang_code() -> String {
    let locale = get_system_locale();
    
    locale
        .split(|c| c == '-' || c == '_')
        .next()
        .unwrap_or("en")
        .to_lowercase()
}