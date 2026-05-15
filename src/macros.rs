#[macro_export]
macro_rules! mre_main {
    ($user_main:path) => {
        #[cfg(target_arch = "arm")]
        #[unsafe(no_mangle)]
        pub extern "C" fn _start(entry: $crate::entry::GetSymEntryFunc,   _init_array_start: usize,  _count: usize ) {
            unsafe {
                $crate::entry::SYSTEM_GET_SYM_ENTRY = Some(entry);
                $user_main(); 
            }
        }

        #[cfg(target_os = "windows")]
        #[unsafe(no_mangle)]
        pub extern "cdecl" fn vm_entry(entry: $crate::entry::GetSymEntryFunc) {
            unsafe {
                $crate::entry::SYSTEM_GET_SYM_ENTRY = Some(entry);
                $user_main(); 
            }
        }
    };
}

#[macro_export]
macro_rules! mre_api {
    ($name:ident($($arg:ident: $arg_ty:ty),*) -> $ret_ty:ty = $fallback:expr) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $name($($arg: $arg_ty),*) -> $ret_ty {
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