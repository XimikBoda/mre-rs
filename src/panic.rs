use crate::ffi::app::vm_exit_app;

pub static mut ACTIVE_JUMP_POINT: *const () = core::ptr::null();
pub static mut ACTIVE_JUMP_CALL: Option<unsafe fn(*const (), usize)> = None;

static mut IS_PANICKING: bool = false;

pub static mut CRASH_LOG: [u8; 512] = [0; 512];
pub static mut CRASH_LOG_LEN: usize = 0;

fn soft_reset() -> ! {
    unsafe {
        let soft_reset: extern "C" fn() -> ! = core::mem::transmute(0_usize as *const ());
            
        soft_reset();
    }
}

fn trigger_bsod_and_exit() {
    unsafe { vm_exit_app();}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        if IS_PANICKING {
            soft_reset();
        }
        IS_PANICKING = true;

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