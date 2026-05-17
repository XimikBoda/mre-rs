use alloc::vec::Vec;
use crate::ffi::pmng::*;
use crate::ffi::ucs2::{from_ucs2};
use crate::fs::path::Path;
use crate::msg::MsgError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Unload,
    Foreground,
    Background,
    Starting,
    Closing,
    Inactive,
    Unknown,
}

impl ProcessState {
    pub(crate) fn from_i32(state: i32) -> Self {
        match state {
            VM_PMNG_UNLOAD => ProcessState::Unload,
            VM_PMNG_FOREGROUND => ProcessState::Foreground,
            VM_PMNG_BACKGROUND => ProcessState::Background,
            VM_PMNG_STARTING => ProcessState::Starting,
            VM_PMNG_CLOSING => ProcessState::Closing,
            VM_PMNG_INACTIVE => ProcessState::Inactive,
            _ => ProcessState::Unknown,
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Process(VM_P_HANDLE);

pub struct ProcessProperty {
    pub handle: Process,
    pub app_id: i32,
    pub status: ProcessState,
    pub parent: Option<Process>,
    pub file_path: Path,
}

static mut CURRENT_HANDLE: VM_P_HANDLE = -1;

impl Process {
    pub fn current() -> Option<Self> {
        unsafe {
            if CURRENT_HANDLE <= 0 {
                let handle = vm_pmng_get_current_handle();
                if handle > 0 {
                    CURRENT_HANDLE = handle;
                } else {
                    return None;
                }
            }
            Some(Self(CURRENT_HANDLE))
        }
    }

    pub(crate) fn from_handle(handle: VM_P_HANDLE) -> Option<Self> {
        if handle > 0 { Some(Self(handle)) } else { None }
    }

    pub fn by_path(path: &Path) -> Option<Self> {
        let ucs2_path = path.as_mre_str();
        let handle = unsafe { vm_pmng_get_handle(ucs2_path.as_ptr()) };
        Self::from_handle(handle)
    }

    pub(crate) fn handle(&self) -> VM_P_HANDLE { self.0 }

    pub fn state(&self) -> ProcessState {
        let state = unsafe { vm_pmng_state(self.0) };
        ProcessState::from_i32(state)
    }

    pub fn property(&self) -> Option<ProcessProperty> {
        let mut prop = unsafe { core::mem::zeroed::<vm_process_property>() };
        let res = unsafe { vm_pmng_get_process_property(self.0, &mut prop) };

        if res < 0 {
            return None;
        }

        let path_str = from_ucs2(&prop.filePath);

        Some(ProcessProperty {
            handle: *self,
            app_id: prop.appID,
            status: ProcessState::from_i32(prop.status),
            parent: Self::from_handle(prop.pParentHandle),
            file_path: Path::new(&path_str),
        })
    }

    pub fn send_msg(&self, msg_id: u32, wparam: i32, lparam: i32) -> Result<i32, MsgError> {
        if let Some(current) = Process::current() {
            if self.0 == current.0 {
                return Err(MsgError::TargetIsSelf);
            }
        }

        if msg_id < crate::ffi::msg::VM_MESSAGE_ID_BASE {
            return Err(MsgError::InvalidMsgId);
        }

        let res = unsafe { crate::ffi::msg::vm_send_msg(self.0, msg_id, wparam, lparam) };
        Ok(res)
    }

    pub fn post_msg(&self, msg_id: u32, wparam: i32, lparam: i32) -> Result<(), MsgError> {
        if let Some(current) = Process::current() {
            if self.0 == current.0 {
                return Err(MsgError::TargetIsSelf);
            }
        }

        if msg_id < crate::ffi::msg::VM_MESSAGE_ID_BASE {
            return Err(MsgError::InvalidMsgId);
        }

        let res = unsafe { crate::ffi::msg::vm_post_msg(self.0, msg_id, wparam, lparam) };
        
        if res == 1 {
            Ok(())
        } else {
            Err(MsgError::QueueFull)
        }
    }

    pub fn spawn(path: &Path, exit_current: bool) {
        let ucs2_path = path.as_mre_str();

        let parent_handle = Self::current().map(|p| p.0).unwrap_or(0);

        unsafe {
            vm_start_app(ucs2_path.as_ptr(), parent_handle, exit_current as i32);
        }
    }
}

pub fn list_processes() -> Vec<Process> {
    let mut num: u32 = 0;
    
    unsafe { vm_pmng_get_process_list(core::ptr::null_mut(), &mut num) };
    
    if num == 0 {
        return Vec::new();
    }

    let mut handles = alloc::vec![0 as VM_P_HANDLE; num as usize];
    
    let res = unsafe { vm_pmng_get_process_list(handles.as_mut_ptr(), &mut num) };
    
    if res < 0 {
        return Vec::new();
    }

    handles.into_iter()
           .filter_map(Process::from_handle)
           .collect()
}