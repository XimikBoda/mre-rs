use core::ffi::c_void;
use embedded_io::{Error, ErrorKind, ErrorType, Read, Seek, SeekFrom, Write};
use crate::fs::path::Path;
use crate::ffi::file::*;

#[derive(Debug, Clone, Copy)]
pub struct MreIoError(pub i32);

impl Error for MreIoError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
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
        let handle = unsafe { vm_file_open(ucs2_path.as_ptr(), mode, 1) };
        
        if handle < 0 { Err(handle) } else { Ok(Self { handle }) }
    }

    pub fn size(&self) -> Result<usize, i32> {
        let mut size: u32 = 0;
        let res = unsafe { vm_file_getfilesize(self.handle, &mut size) };
        if res < 0 { Err(res) } else { Ok(size as usize) }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { vm_file_close(self.handle); }
    }
}

impl ErrorType for File {
    type Error = MreIoError;
}

impl Read for File {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        let mut nread: u32 = 0;
        let res = unsafe {
            vm_file_read(
                self.handle,
                buf.as_mut_ptr() as *mut c_void,
                buf.len() as u32,
                &mut nread,
            )
        };

        if res < 0 {
            Err(MreIoError(res))
        } else {
            Ok(nread as usize)
        }
    }
}

impl Write for File {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        let mut written: u32 = 0;
        let res = unsafe {
            vm_file_write(
                self.handle,
                buf.as_ptr() as *mut c_void,
                buf.len() as u32,
                &mut written,
            )
        };

        if res < 0 {
            Err(MreIoError(res))
        } else {
            Ok(written as usize)
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        let res = unsafe { vm_file_commit(self.handle) };
        if res < 0 { Err(MreIoError(res)) } else { Ok(()) }
    }
}

impl Seek for File {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Self::Error> {
        let (offset, base) = match pos {
            SeekFrom::Start(off) => (off as i32, VM_FS_BASE_BEGIN),
            SeekFrom::Current(off) => (off as i32, VM_FS_BASE_CURR),
            SeekFrom::End(off) => (off as i32, VM_FS_BASE_END),
        };

        let res = unsafe { vm_file_seek(self.handle, offset, base) };
        if res < 0 {
            return Err(MreIoError(res));
        }

        let current_pos = unsafe { vm_file_tell(self.handle) };
        if current_pos < 0 {
            Err(MreIoError(current_pos))
        } else {
            Ok(current_pos as u64)
        }
    }
}