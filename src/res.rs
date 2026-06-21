extern crate alloc;
use alloc::vec::Vec;
use core::ops::Deref;
use core::{mem, slice};
use crate::ffi::res::*;
use crate::ffi::mem::vm_free;

pub struct ResourceData {
    ptr: *mut u8,
    size: usize,
}

impl ResourceData {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.size) }
    }
}

impl Deref for ResourceData {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl Drop for ResourceData {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                vm_free(self.ptr as *mut core::ffi::c_void);
            }
        }
    }
}

fn to_c_string(s: &str) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(s.len() + 1);
    bytes.extend_from_slice(s.as_bytes());
    bytes.push(0);
    bytes
}

pub fn load_resource(res_name: &str) -> Option<ResourceData> {
    let c_name = to_c_string(res_name);
    let mut size: i32 = 0;

    let ptr = unsafe { 
        vm_load_resource(c_name.as_ptr(), &mut size) 
    };

    if ptr.is_null() || size <= 0 {
        return None;
    }

    Some(ResourceData {
        ptr,
        size: size as usize,
    })
}

pub fn get_resource_offset(res_name: &str) -> Option<u32> {
    let c_name = to_c_string(res_name);
    let offset = unsafe { vm_get_resource_offset(c_name.as_ptr()) };
    
    if offset == 0 {
        None
    } else {
        Some(offset)
    }
}

pub fn read_resource_chunk<T>(offset: u32, buffer: &mut [T]) -> Result<(), ()> {
    let byte_len = buffer.len() * mem::size_of::<T>();
    
    let ptr = buffer.as_mut_ptr() as *mut u8;

    let result = unsafe {
        vm_resource_get_data(ptr, offset, byte_len as u32)
    };

    if result == 0 {
        Ok(())
    } else {
        Err(())
    }
}