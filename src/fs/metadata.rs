use crate::fs::path::{Path};
use crate::fs::attributes::FileAttributes;
use crate::ffi::fs::*;
use crate::time::datetime::DateTime;

#[derive(Clone, Debug)]
pub struct Metadata {
    pub size: u32,
    pub attributes: FileAttributes,
    pub created: DateTime,
    pub modified: DateTime,
}

impl Metadata {
    pub fn is_dir(&self) -> bool {
        self.attributes.is_dir()
    }
    
    pub fn is_file(&self) -> bool {
        !self.is_dir()
    }
}

pub fn metadata(path: &Path) -> Result<Metadata, i32> {
    let ucs2_path = path.as_mre_str();
    let mut info: vm_fileinfo_ext = unsafe { core::mem::zeroed() };

    let handle = unsafe{ vm_find_first_ext(ucs2_path.as_ptr(), &mut info) };

    if handle < 0 {
        return Err(handle);
    }

    unsafe{ vm_find_close_ext(handle) };

    Ok(Metadata {
        size: info.filesize,
        attributes: FileAttributes(info.attributes),
        created: info.create_datetime.into(),
        modified: info.modify_datetime.into(),
    })
}