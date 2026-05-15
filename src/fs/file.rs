use core::ffi::c_void;
use crate::fs::path::Path;
use crate::sys::file::*;

#[derive(Clone, Copy, Debug)]
pub enum SeekFrom {
    Start(u32),
    End(i32),
    Current(i32),
}

pub struct File {
    handle: i32,
}

impl File {
    pub fn open(path: &Path) -> Result<Self, i32> {
        Self::open_with_mode(path, VM_FS_MODE_READ)
    }

    pub fn create(path: &Path) -> Result<Self, i32> {
        Self::open_with_mode(path, VM_FS_MODE_CREATE_ALWAYS_WRITE)
    }

    pub fn append(path: &Path) -> Result<Self, i32> {
        Self::open_with_mode(path, VM_FS_MODE_APPEND)
    }

    fn open_with_mode(path: &Path, mode: u32) -> Result<Self, i32> {
        let ucs2_path = path.as_mre_str();
        
        let handle = unsafe{ vm_file_open(ucs2_path.as_ptr(), mode, 1) };
        
        if handle < 0 {
            Err(handle)
        } else {
            Ok(Self { handle })
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, i32> {
        let mut nread: u32 = 0;
        let res = unsafe{ 
            vm_file_read(
                self.handle,
                buf.as_mut_ptr() as *mut c_void,
                buf.len() as u32,
                &mut nread,
            )
        };

        if res < 0 {
            Err(res)
        } else {
            Ok(nread as usize)
        }
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<usize, i32> {
        let mut written: u32 = 0;
        let res = unsafe{ 
            vm_file_write(
                self.handle,
                buf.as_ptr() as *mut c_void,
                buf.len() as u32,
                &mut written,
            )
        };

        if res < 0 {
            Err(res)
        } else {
            Ok(written as usize)
        }
    }

    pub fn seek(&mut self, pos: SeekFrom) -> Result<(), i32> {
        let (offset, base) = match pos {
            SeekFrom::Start(off) => (off as i32, VM_FS_BASE_BEGIN),
            SeekFrom::Current(off) => (off, VM_FS_BASE_CURR),
            SeekFrom::End(off) => (off, VM_FS_BASE_END),
        };

        let res = unsafe{ vm_file_seek(self.handle, offset, base) };
        if res < 0 { Err(res) } else { Ok(()) }
    }

    pub fn tell(&self) -> Result<usize, i32> {
        let res = unsafe{ vm_file_tell(self.handle) };
        if res < 0 { Err(res) } else { Ok(res as usize) }
    }

    pub fn commit(&self) -> Result<(), i32> {
        let res = unsafe{ vm_file_commit(self.handle) };
        if res != 0 { Err(res) } else { Ok(()) }
    }

    pub fn is_eof(&self) -> bool {
        let res = unsafe{ vm_file_is_eof(self.handle) };
        res != 0
    }

    pub fn size(&self) -> Result<usize, i32> {
        let mut size: u32 = 0;
        let res = unsafe{ vm_file_getfilesize(self.handle, &mut size) };
        if res < 0 { Err(res) } else { Ok(size as usize) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe{ vm_file_close(self.handle) };
    }
}