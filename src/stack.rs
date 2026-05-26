#[cfg(all(feature = "custom-stack", target_arch = "arm"))]
pub static mut CUSTOM_STACK_TOP: usize = 0;

#[cfg(all(feature = "custom-stack", target_arch = "arm"))]
pub static mut CUSTOM_STACK_BOTTOM: usize = 0;

#[cfg(all(feature = "custom-stack", target_arch = "arm"))]
static mut STACK_STORAGE: Option<alloc::vec::Vec<u8>> = None;

#[cfg(all(feature = "custom-stack", target_arch = "arm"))]
pub unsafe fn init(size: usize) {
    let vec = alloc::vec![0u8; size];
    
    let bottom = vec.as_ptr() as usize;
    let mut top = bottom + size;
    
    top &= !7; 
    
    unsafe {
        CUSTOM_STACK_BOTTOM = bottom;
        CUSTOM_STACK_TOP = top;
        STACK_STORAGE = Some(vec);
    }
}

#[cfg(all(feature = "custom-stack", target_arch = "arm"))]
#[inline(never)]
pub unsafe fn run_on_custom_stack<R, F: FnOnce() -> R>(f: F) -> R {
    unsafe {
        let top = CUSTOM_STACK_TOP;
        let bottom = CUSTOM_STACK_BOTTOM;

        if top == 0 {
            return f();
        }

        let current_sp: usize;
        core::arch::asm!("mov {}, sp", out(reg) current_sp);
        if current_sp >= bottom && current_sp <= top {
            return f(); 
        }

        let mut result: Option<R> = None;
        let mut f_opt = Some(f);
        
        let mut wrapper = || {
            let func = f_opt.take().unwrap_unchecked();
            result = Some(func());
        };
        
        let wrapper_ptr = &mut wrapper as *mut _ as usize;

        #[inline(always)]
        fn get_trampoline<C: FnMut()>(_: &C) -> extern "C" fn(usize) {
            extern "C" fn trampoline<C: FnMut()>(ptr: usize) {
                let closure_ref = unsafe { &mut *(ptr as *mut C) };
                closure_ref();
            }
            trampoline::<C> 
        }
        
        let trampoline_fn = get_trampoline(&wrapper);

        core::arch::asm!(
            "mov r4, sp",          
            "mov sp, {top}",       
            
            "blx {trampoline}",    
            
            "mov sp, r4",          
            
            top = in(reg) top,
            trampoline = in(reg) trampoline_fn,
            inout("r0") wrapper_ptr => _,
            out("r4") _,
            clobber_abi("C")
        );

        result.unwrap_unchecked()
    }
}

#[cfg(not(all(feature = "custom-stack", target_arch = "arm")))]
pub unsafe fn init(_size: usize) {
}

#[cfg(not(all(feature = "custom-stack", target_arch = "arm")))]
#[inline(always)]
pub unsafe fn run_on_custom_stack<R, F: FnOnce() -> R>(f: F) -> R {
    f()
}