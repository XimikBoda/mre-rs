#![allow(unused_assignments)]
#![allow(unused_unsafe)]
use core::ffi::c_void;
use core::char::decode_utf16;
use core::fmt::{self, Write};
use crate::stack::{get_current_fp, check_frame_pointers_working};

const CRASH_FILE_PATH: [u16; 18] = [
    'e' as u16, ':' as u16, '\\' as u16, 
    'r' as u16, 'u' as u16, 's' as u16, 't' as u16, '_' as u16, 
    'c' as u16, 'r' as u16, 'a' as u16, 's' as u16, 'h' as u16, '.' 
    as u16, 't' as u16, 'x' as u16, 't' as u16, 0
];

const CRASH_VIEWER_PATH: [u16; 29] = [
    'e' as u16, ':' as u16, '\\' as u16, 
    'm' as u16, 'r' as u16, 'e' as u16, '\\' as u16, 
    'r' as u16, 'u' as u16, 's' as u16, 't' as u16, '_' as u16, 
    'p' as u16, 'a' as u16, 'n' as u16, 'i' as u16, 'c' as u16, '_' as u16,
    'v' as u16, 'i' as u16, 'e' as u16, 'w' as u16, 'e' as u16, 'r' as u16, '.' as u16,
    'v' as u16, 'x' as u16, 'p' as u16, 0
];

#[cfg(target_arch = "arm")]
const ARCH_STR: &str = "arm";

#[cfg(target_arch = "x86")]
const ARCH_STR: &str = "x86";

pub static mut ACTIVE_JUMP_POINT: *const () = core::ptr::null();
pub static mut ACTIVE_JUMP_CALL: Option<unsafe fn(*const (), usize)> = None;

pub static mut RUNTIME_START_ADDR: usize = 0;

pub static mut PANIC_STAGE: u8 = 0;

struct AppPathZeroAlloc;

impl fmt::Display for AppPathZeroAlloc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = [0u16; 260];
        
        let result = unsafe { crate::ffi::fs::vm_get_exec_filename(buffer.as_mut_ptr()) };
        
        if result < 0 {
            return write!(f, "unknown_path");
        }

        let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());

        for decoded_char in decode_utf16(buffer[..len].iter().copied()) {
            let ch = decoded_char.unwrap_or('?');
            write!(f, "{}", ch)?;
        }
        
        Ok(())
    }
}

struct CrashLogger {
    handle: i32,
}

impl Write for CrashLogger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.handle < 0 {
            return Ok(());
        }

        let mut written: u32 = 0;
        unsafe {
            crate::ffi::file::vm_file_write(
                self.handle,
                s.as_ptr() as *mut c_void,
                s.len() as u32,
                &mut written,
            );
        }
        
        Ok(())
    }
}

fn soft_reset() -> ! {
    unsafe {
        let soft_reset: extern "C" fn() -> ! = core::mem::transmute(0_usize as *const ());
            
        soft_reset();
    }
}

fn trigger_bsod_and_exit() {
unsafe {
        let _ = crate::sjlj2::catch_long_jump(|jump_point| {
            unsafe fn call_jump(jp_ptr: *const (), payload: usize) {
                unsafe {
                    let jp = &*(jp_ptr as *const crate::sjlj2::JumpPoint);    
                    jp.long_jump(payload);
                }
            }
            
            ACTIVE_JUMP_POINT = &jump_point as *const _ as *const ();
            ACTIVE_JUMP_CALL = Some(call_jump);
            
            crate::app::run_atexit_hooks();
            
            ACTIVE_JUMP_POINT = core::ptr::null();
            ACTIVE_JUMP_CALL = None;
        });

        ACTIVE_JUMP_POINT = core::ptr::null();
        ACTIVE_JUMP_CALL = None;
        
        PANIC_STAGE = 2; 

        
        crate::ffi::pmng::vm_start_app(CRASH_VIEWER_PATH.as_ptr(),0, 0);
        crate::ffi::app::vm_exit_app();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if PANIC_STAGE >= 2 {
            soft_reset();
        }

        if PANIC_STAGE == 0 {
            PANIC_STAGE = 1;
            {
                let handle = crate::ffi::file::vm_file_open(
                    CRASH_FILE_PATH.as_ptr(), 
                    crate::ffi::file::VM_FS_MODE_CREATE_ALWAYS_WRITE, 
                    1
                );

                if handle >= 0 {
                    let mut logger = CrashLogger { handle };

                    if let Some(location) = info.location() {
                        let _ = write!(logger, "File:{}\nLine:{}\n", location.file(), location.line());
                    } else {
                        let _ = write!(logger, "File:unknown\nLine:0\n");
                    }

                    let _ = write!(logger, "App:{}\n", AppPathZeroAlloc);

                    let _ = write!(logger, "Arch:{}\n", ARCH_STR);

                    let start_addr = RUNTIME_START_ADDR;
                    let _ = write!(logger, "RuntimeStart:0x{:08X}\n", start_addr);

                    let limit_addr = crate::stack::STACK_LIMIT_ADDR;
                
                    if !check_frame_pointers_working() {
                        let _ = write!(logger, "Backtrace:Unavailable (Frame Pointers disabled)\n");
                    } else if limit_addr == 0 {
                        let _ = write!(logger, "Backtrace:Unavailable (Stack limit unknown)\n");
                    } else {
                        let mut current_fp = get_current_fp();
                        let mut depth = 0;
                        
                        loop {
                            if current_fp >= limit_addr {
                                break;
                            }
                            if current_fp == 0 || current_fp % 4 != 0 {
                                let _ = write!(logger, "Backtrace:{}:Corrupted (FP=0x{:08X})\n", depth, current_fp);
                                break;
                            }
                            if depth >= 64 {
                                let _ = write!(logger, "Backtrace:Truncated (Max depth reached)\n");
                                break;
                            }

                            let ret_addr = *(current_fp as *const usize).add(1);
                            let _ = write!(logger, "Backtrace:{}:0x{:08X}\n", depth, ret_addr);
                            
                            current_fp = *(current_fp as *const usize);
                            depth += 1;
                        }
                    }

                    let _ = write!(logger, "Msg:{}\n", info.message());

                    crate::ffi::file::vm_file_commit(handle); 
                    
                    crate::ffi::file::vm_file_close(handle);
                }
            }
        } else if PANIC_STAGE == 1 {
            PANIC_STAGE = 2;
        }

        if let Some(jumper_fn) = ACTIVE_JUMP_CALL {
            if !ACTIVE_JUMP_POINT.is_null() {
                jumper_fn(ACTIVE_JUMP_POINT, 1);
            }
        }

        soft_reset();
    }
}

#[inline(always)]
pub fn with_protection<R, F: FnOnce() -> R>(f: F) -> R {
    let ret = crate::sjlj2::catch_long_jump(|jump_point| {
        unsafe fn call_jump(jp_ptr: *const (), payload: usize) {
            unsafe{
                let jp = &*(jp_ptr as *const crate::sjlj2::JumpPoint);    
                jp.long_jump(payload);
            }
        }
        
        unsafe {
            ACTIVE_JUMP_POINT = &jump_point as *const _ as *const ();
            ACTIVE_JUMP_CALL = Some(call_jump);
        }
        
        let result = f();
        
        unsafe {
            ACTIVE_JUMP_POINT = core::ptr::null();
            ACTIVE_JUMP_CALL = None;
        }
        
        result
    });

    match ret {
        core::ops::ControlFlow::Continue(val) => val,
        core::ops::ControlFlow::Break(_) => {
            trigger_bsod_and_exit();
            unsafe { core::mem::zeroed::<R>() }
        }
    }
}
