use alloc::boxed::Box;
use crate::fs::path::Path;
use crate::ffi::fs::*;

#[derive(Debug, Clone, Copy)]
pub enum CopyStatus {
    Fail,
    Prepare,
    Start,
    Progress { total: u32, completed: u32 },
    Done,
}

type CopyHandler = Box<dyn FnMut(CopyStatus, i32) -> bool>;

static mut ACTIVE_COPIES: [Option<(i32, CopyHandler)>; 4] = [None, None, None, None];

extern "C" fn global_copy_cb(act: i32, total: u32, completed: u32, hdl: i32) -> i32 {
    let status = match act {
        VM_FS_MOVE_PGS_FAIL => CopyStatus::Fail,
        VM_FS_MOVE_PGS_PREPARE => CopyStatus::Prepare,
        VM_FS_MOVE_PGS_START => CopyStatus::Start,
        VM_FS_MOVE_PGS_ING => CopyStatus::Progress { total, completed },
        VM_FS_MOVE_PGS_DONE => CopyStatus::Done,
        _ => return 0,
    };

    unsafe {
        let copies = &mut *core::ptr::addr_of_mut!(ACTIVE_COPIES);

        if act == VM_FS_MOVE_PGS_PREPARE {
            for slot in copies.iter_mut() {
                if let Some((stored_hdl, _)) = slot {
                    if *stored_hdl == -1 {
                        *stored_hdl = hdl;
                        break;
                    }
                }
            }
        }

        for slot in copies.iter_mut() {
            if let Some((stored_hdl, handler)) = slot {
                if *stored_hdl == hdl {
                    let continue_copy = handler(status, hdl);

                    if !continue_copy || act == VM_FS_MOVE_PGS_DONE || act == VM_FS_MOVE_PGS_FAIL {
                        *slot = None;
                    }
                    
                    return if continue_copy { 0 } else { -1 };
                }
            }
        }
    }
    0
}

pub fn copy_file_async<F>(src: &Path, dst: &Path, callback: F) -> Result<(), i32>
where
    F: FnMut(CopyStatus, i32) -> bool + 'static,
{
    unsafe {
        let mut slot_found = false;
        let copies = &mut *core::ptr::addr_of_mut!(ACTIVE_COPIES);

        for slot in copies.iter_mut() {
            if slot.is_none() {
                *slot = Some((-1, Box::new(callback)));
                slot_found = true;
                break;
            }
        }

        if !slot_found {
            return Err(-46);
        }

        let ucs2_src = src.as_mre_str();
        let ucs2_dst = dst.as_mre_str();

        let res = vm_file_copy(ucs2_dst.as_ptr(), ucs2_src.as_ptr(), global_copy_cb);

        if res < 0 {
            for slot in copies.iter_mut() {
                if let Some((hdl, _)) = slot {
                    if *hdl == -1 { *slot = None; break; }
                }
            }
            Err(res)
        } else {
            Ok(())
        }
    }
}

pub fn abort_copy(hdl: i32) {
    unsafe{ vm_file_copy_abort(hdl) };
}