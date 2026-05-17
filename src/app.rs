use alloc::boxed::Box;
use crate::ffi::app::*;
use crate::ffi::pmng::{vm_pmng_set_bg, vm_pmng_set_fg};
use crate::process::{Process, ProcessState};

pub fn current_process() -> Option<Process> {
    Process::current()
}

pub fn current_state() -> ProcessState {
    Process::current()
        .map(|p| p.state())
        .unwrap_or(ProcessState::Unknown)
}

pub fn is_active() -> bool {
    current_state() == ProcessState::Foreground
}

pub fn is_background() -> bool {
    current_state() == ProcessState::Background
}


#[derive(Debug, Clone)]
pub enum Event {
    Create { param: i32 },
    Paint,
    Active,
    Inactive,
    Hide,
    Quit,
    CardPlugOut,
    ScreenRotate,
    Push,
    Unknown(i32, i32),
}

type EventHandler = Box<dyn FnMut(Event)>;
static mut APP_HANDLER: Option<EventHandler> = None;

const MAX_SUBSYSTEMS: usize = 8;
static mut ATEXIT_HOOKS: [Option<fn()>; MAX_SUBSYSTEMS] = [None; MAX_SUBSYSTEMS];

static mut SYS_CALLBACK_REGISTERED: bool = false;

fn ensure_sys_callback() {
    unsafe {
        if !SYS_CALLBACK_REGISTERED {
            vm_reg_sysevt_callback(global_sysevt_router);
            SYS_CALLBACK_REGISTERED = true;
        }
    }
}

extern "C" fn global_sysevt_router(message: i32, param: i32) {
    let event = match message {
        VM_MSG_CREATE => Event::Create { param },
        VM_MSG_PAINT => Event::Paint,
        VM_MSG_ACTIVE => Event::Active,
        VM_MSG_INACTIVE => Event::Inactive,
        VM_MSG_HIDE => Event::Hide,
        VM_MSG_QUIT => Event::Quit,
        VM_MSG_CARD_PLUG_OUT => Event::CardPlugOut,
        VM_MSG_SCREEN_ROTATE => Event::ScreenRotate,
        VM_MSG_PUSH => Event::Push,
        _ => Event::Unknown(message, param),
    };

    if let Event::Quit = event {
        let handler_ptr = core::ptr::addr_of_mut!(APP_HANDLER);

        let handler_opt = unsafe { core::ptr::replace(handler_ptr, None) };

        if let Some(mut handler) = handler_opt {
            handler(Event::Quit);
        }

        unsafe {
            let hooks = &*core::ptr::addr_of!(ATEXIT_HOOKS);

            for hook in hooks.iter().flatten() {
                hook();
            }
        }

    } else {
        unsafe {
            let handler_ptr = core::ptr::addr_of_mut!(APP_HANDLER);
            if let Some(handler) = (*handler_ptr).as_mut() {
                handler(event);
            }
        }
    }
}

pub fn set_handler<F>(handler: F)
where
    F: FnMut(Event) + 'static,
{
    ensure_sys_callback();
    
    unsafe {
        let handler_ptr = core::ptr::addr_of_mut!(APP_HANDLER);
        *handler_ptr = Some(Box::new(handler));
    }
}

pub fn register_atexit(hook: fn()) {
    ensure_sys_callback(); 
    
    unsafe {
        let hooks = &mut *core::ptr::addr_of_mut!(ATEXIT_HOOKS);

        if hooks.iter().flatten().any(|&existing| existing as usize == hook as usize) {
            return;
        }
        
        if let Some(slot) = hooks.iter_mut().find(|slot| slot.is_none()) {
            *slot = Some(hook);
        }
    }
}

pub fn exit() {
    global_sysevt_router(VM_MSG_QUIT, 0);
    unsafe { vm_exit_app() };
}

pub fn flush_screen() {
    unsafe { vm_graphic_flush_screen() };
}

pub fn set_background() -> Result<(), i32> {
    let res = unsafe { vm_pmng_set_bg() };
    if res == 0 { Ok(()) } else { Err(res) }
}

pub fn set_foreground() -> Result<(), i32> {
    let prop = Process::current()
        .and_then(|p| p.property())
        .ok_or(-1)?;
    
    let ucs2_name = prop.file_path.as_mre_str();
    let res = unsafe { vm_pmng_set_fg(ucs2_name.as_ptr()) };
    
    if res < 0 { Err(res) } else { Ok(()) }
}