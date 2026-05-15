use crate::fs::path::Path;
use crate::sys::fs::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FileAttributes(pub u8);

impl FileAttributes {
    pub const READ_ONLY: u8 = 0x01;
    pub const HIDDEN:    u8 = 0x02;
    pub const SYSTEM:    u8 = 0x04;
    pub const VOLUME:    u8 = 0x08;
    pub const DIR:       u8 = 0x10;
    pub const ARCHIVE:   u8 = 0x20;

    pub fn empty() -> Self {
        Self(0)
    }

    pub fn is_read_only(&self) -> bool { (self.0 & Self::READ_ONLY) != 0 }
    pub fn is_hidden(&self) -> bool { (self.0 & Self::HIDDEN) != 0 }
    pub fn is_system(&self) -> bool { (self.0 & Self::SYSTEM) != 0 }
    pub fn is_dir(&self) -> bool { (self.0 & Self::DIR) != 0 }
    pub fn is_archive(&self) -> bool { (self.0 & Self::ARCHIVE) != 0 }

    pub fn set_read_only(&mut self, val: bool) {
        if val { self.0 |= Self::READ_ONLY; } else { self.0 &= !Self::READ_ONLY; }
    }

    pub fn set_hidden(&mut self, val: bool) {
        if val { self.0 |= Self::HIDDEN; } else { self.0 &= !Self::HIDDEN; }
    }

    pub fn set_archive(&mut self, val: bool) {
        if val { self.0 |= Self::ARCHIVE; } else { self.0 &= !Self::ARCHIVE; }
    }
}

pub fn get_attributes(path: &Path) -> Result<FileAttributes, i32> {
    let ucs2_path = path.as_mre_str();
    let res = vm_file_get_attributes(ucs2_path.as_ptr());
    
    if res == -1 {
        Err(res)
    } else {
        Ok(FileAttributes(res as u8))
    }
}

pub fn set_attributes(path: &Path, attrs: FileAttributes) -> Result<(), i32> {
    let ucs2_path = path.as_mre_str();
    
    let safe_attrs = attrs.0 & !(FileAttributes::VOLUME | FileAttributes::DIR);
    
    let res = vm_file_set_attributes(ucs2_path.as_ptr(), safe_attrs);
    
    if res != 0 {
        Err(res)
    } else {
        Ok(())
    }
}