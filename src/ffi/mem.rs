use core::ffi::{c_void};
use crate::mre_api;

//malloc_stat_t* vm_get_malloc_stat(void);

mre_api!(vm_malloc(size: i32) -> *mut c_void);
mre_api!(vm_calloc(size: i32) -> *mut c_void);
mre_api!(vm_realloc(ptr: *mut c_void, size: i32) -> *mut c_void);
mre_api!(vm_free(ptr: *mut c_void));

mre_api!(vm_global_malloc(size: u32) -> *mut c_void);
mre_api!(vm_global_free(ptr: *mut c_void));
mre_api!(vm_global_get_max_alloc_size() -> i32 = -1);

mre_api!(vm_malloc_nc(size: i32) -> *mut c_void);
mre_api!(vm_malloc_nc_topmost(size: i32) -> *mut c_void);
mre_api!(vm_malloc_topmost(size: i32) -> *mut c_void);
mre_api!(vm_realloc_topmost(ptr: *mut c_void, size: i32) -> *mut c_void);
