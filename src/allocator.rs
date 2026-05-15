use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use crate::sys::mem::*;

struct MreAllocator;

unsafe impl GlobalAlloc for MreAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe{ vm_malloc(layout.size() as i32) as *mut u8 }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe{ vm_calloc(layout.size() as i32) as *mut u8 }
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 { 
        unsafe{ vm_realloc(ptr as *mut c_void,  new_size as i32) as *mut u8 }
     }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe{ vm_free(ptr as *mut c_void) }
    }
}

#[global_allocator]
static ALLOCATOR: MreAllocator = MreAllocator;

