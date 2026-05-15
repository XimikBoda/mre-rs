use crate::fs::path::Path;
use crate::sys::fs::*;

use crate::fs::metadata::metadata;

pub fn remove_file(path: &Path) -> Result<(), i32> {
    let ucs2_path = path.as_mre_str();
    let res = vm_file_delete(ucs2_path.as_ptr());
    
    if res < 0 { Err(res) } else { Ok(()) }
}

pub fn rename(from: &Path, to: &Path) -> Result<(), i32> {
    let ucs2_from = from.as_mre_str();
    let ucs2_to = to.as_mre_str();
    
    let res = vm_file_rename(ucs2_from.as_ptr(), ucs2_to.as_ptr());
    
    if res < 0 { Err(res) } else { Ok(()) }
}

pub fn create_dir(path: &Path) -> Result<(), i32> {
    let ucs2_path = path.as_mre_str();
    let res = vm_file_mkdir(ucs2_path.as_ptr());
    
    if res < 0 { Err(res) } else { Ok(()) }
}

pub fn remove_dir(path: &Path) -> Result<(), i32> {
    let ucs2_path = path.as_mre_str();
    let res = vm_file_rmdir(ucs2_path.as_ptr());
    
    if res < 0 { Err(res) } else { Ok(()) }
}

pub fn exists(path: &Path) -> bool {
    metadata(path).is_ok()
}