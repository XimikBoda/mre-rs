use crate::ffi::app::vm_exit_app;
use core::char::decode_utf16;
use core::fmt::{self, Write};

const CRASH_FILE_PATH: [u16; 18] = [
    'e' as u16, ':' as u16, '\\' as u16, 
    'r' as u16, 'u' as u16, 's' as u16, 't' as u16, '_' as u16, 
    'c' as u16, 'r' as u16, 'a' as u16, 's' as u16, 'h' as u16, '.' 

pub static mut ACTIVE_JUMP_POINT: *const () = core::ptr::null();
pub static mut ACTIVE_JUMP_CALL: Option<unsafe fn(*const (), usize)> = None;

pub static mut PANIC_STAGE: u8 = 0;

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

        vm_exit_app();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if PANIC_STAGE >= 2 {
            soft_reset();
        }

        if PANIC_STAGE == 0 {
            PANIC_STAGE = 1;
            {
                // TODO: 
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

#[macro_export]
macro_rules! protect_call {
    ($body:expr) => {
        $crate::panic::with_protection(|| { $body })
    };
}

#[macro_export]
macro_rules! mre_callback {
    (
        $(#[$attr:meta])* 
        $vis:vis extern "C" fn $name:ident($($arg:ident: $arg_ty:ty),*) $(-> $ret:ty)? {
            $($body:tt)*
        }
    ) => {
        $(#[$attr])*
        $vis extern "C" fn $name($($arg: $arg_ty),*) $(-> $ret)? {
            $crate::panic::with_protection(|| {
                $($body)*
            })
        }
    };
}