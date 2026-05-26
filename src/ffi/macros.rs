#[macro_export]
macro_rules! mre_api {
    ($name:ident($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty = $fallback:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name($($arg: $arg_ty),*) -> $ret_ty {
            static mut FUNC_PTR: *mut core::ffi::c_void = core::ptr::null_mut();
            
            unsafe {
                if FUNC_PTR.is_null() {
                    if let Some(get_sym) = $crate::entry::SYSTEM_GET_SYM_ENTRY {
                        let sym_name = concat!(stringify!($name), "\0");
                        FUNC_PTR = get_sym(sym_name.as_ptr());
                    }
                }
                
                if !FUNC_PTR.is_null() {
                    let func: extern "C" fn($($arg_ty),*) -> $ret_ty = core::mem::transmute(FUNC_PTR);
                    func($($arg),*)
                } else {
                    $fallback 
                }
            }
        }
    };

    ($name:ident($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty) => {
        $crate::mre_api!($name($($arg: $arg_ty),*) -> $ret_ty = core::mem::zeroed());
    };
    
    ($name:ident($($arg:ident: $arg_ty:ty),*)) => {
        $crate::mre_api!($name($($arg: $arg_ty),*) -> () = ());
    };
}

#[macro_export]
macro_rules! mre_callback {
    (
        $(#[$attr:meta])* $vis:vis extern "C" fn $name:ident($($arg:ident: $arg_ty:ty),*) $(-> $ret:ty)? {
            $($body:tt)*
        }
    ) => {
        $(#[$attr])*
        $vis extern "C" fn $name($($arg: $arg_ty),*) $(-> $ret)? {
            let stack_anchor = 0usize;
            let __callback_logic = || {
                $crate::panic::with_protection(|| {
                    $($body)*
                })
            };

            unsafe {
                $crate::panic::STACK_LIMIT_ADDR = &stack_anchor as *const _ as usize;
                $crate::stack::run_on_custom_stack(__callback_logic)
            }
        }
    };
}