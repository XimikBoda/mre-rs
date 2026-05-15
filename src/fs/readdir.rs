use crate::fs::path::Path;
use crate::fs::metadata::Metadata;
use crate::fs::attributes::FileAttributes;
use crate::fs::path::from_ucs2;
use crate::ffi::fs::*;

pub struct DirEntry {
    pub path: Path,
    pub metadata: Metadata,
}

pub struct ReadDir {
    handle: i32,
    is_finished: bool,
    first_result: Option<vm_fileinfo_ext>, 
}

impl Iterator for ReadDir {
    type Item = DirEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_finished {
            return None;
        }

        let info = if let Some(first) = self.first_result.take() {
            first 
        } else {
            let mut next_info: vm_fileinfo_ext = unsafe { core::mem::zeroed() };
            let res = unsafe{ vm_find_next_ext(self.handle, &mut next_info) };
            
            if res < 0 {
                self.is_finished = true;
                return None;
            }
            next_info
        };

        let file_path = Path::from_absolute_string(from_ucs2(&info.filefullname));

        Some(DirEntry {
            path: file_path,
            metadata: Metadata {
                size: info.filesize,
                attributes: FileAttributes(info.attributes),
                created: info.create_datetime.into(),
                modified: info.modify_datetime.into(),
            }
        })
    }
}

impl Drop for ReadDir {
    fn drop(&mut self) {
        if self.handle >= 0 {
            unsafe{ vm_find_close_ext(self.handle) };
        }
    }
}

pub fn read_dir(path: &Path) -> Result<ReadDir, i32> {
    read_dir_masked(path, "*.*")
}

pub fn read_dir_masked(path: &Path, mask: &str) -> Result<ReadDir, i32> {
    let search_path = path.join(mask);
    let ucs2_path = search_path.as_mre_str();
    
    let mut info: vm_fileinfo_ext = unsafe { core::mem::zeroed() };
    let handle = unsafe{ vm_find_first_ext(ucs2_path.as_ptr(), &mut info) };

    if handle < 0 {
        return Err(handle);
    }

    Ok(ReadDir {
        handle,
        is_finished: false,
        first_result: Some(info),
    })
}