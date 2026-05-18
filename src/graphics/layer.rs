use alloc::vec::Vec;
use crate::ffi::layer::*;
use crate::ffi::screen::VM_NO_TRANS_COLOR;
use crate::graphics::color::Color;

static mut LAYER_STACK: Option<Vec<i32>> = None;
static mut ZOMBIE_LAYERS: Option<Vec<i32>> = None;

unsafe fn push_layer_handle(handle: i32) {
    unsafe {
        let stack_ptr = core::ptr::addr_of_mut!(LAYER_STACK);
        if (*stack_ptr).is_none() {
            *stack_ptr = Some(Vec::new());
        }
        (*stack_ptr).as_mut().unwrap().push(handle);
    }
}

unsafe fn pop_layer_handle(handle: i32) {
    unsafe {
        let stack_ptr = core::ptr::addr_of_mut!(LAYER_STACK);
        let zombies_ptr = core::ptr::addr_of_mut!(ZOMBIE_LAYERS);

        if let Some(stack) = (*stack_ptr).as_mut() {
            if stack.last() == Some(&handle) {
                vm_graphic_delete_layer(handle);
                stack.pop();

                if (*zombies_ptr).is_none() {
                    *zombies_ptr = Some(Vec::new());
                }
                let zombies = (*zombies_ptr).as_mut().unwrap();

                while let Some(top) = stack.last() {
                    if let Some(pos) = zombies.iter().position(|z| z == top) {
                        let zombie_handle = zombies.remove(pos);
                        vm_graphic_delete_layer(zombie_handle);
                        stack.pop();
                    } else {
                        break;
                    }
                }
            } else {
                if (*zombies_ptr).is_none() {
                    *zombies_ptr = Some(Vec::new());
                }
                (*zombies_ptr).as_mut().unwrap().push(handle);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerMode {
    Base = VM_BASE_LAYER as isize,
    Fast = VM_FAST_LAYER as isize,
    Buffer = VM_BUF as isize,
    CreateCanvas = VM_CREATE_CANVAS as isize,
}

pub struct Layer {
    handle: i32,
    width: i32,
    height: i32,
}

impl Layer {
    pub fn create(x: i32, y: i32, width: i32, height: i32, trans_color: Option<crate::graphics::color::Color>) -> Result<Self, i32> {
        let tc = trans_color.map(|c| c.0 as i32).unwrap_or(VM_NO_TRANS_COLOR);
        
        let handle = unsafe { vm_graphic_create_layer(x, y, width, height, tc) };
        if handle < 0 {
            return Err(handle);
        }

        unsafe { push_layer_handle(handle); }

        Ok(Self { handle, width, height })
    }

    pub fn create_ex(
        x: i32, y: i32, width: i32, height: i32, 
        trans_color: Option<crate::graphics::color::Color>, 
        mode: LayerMode, 
        buffer: *mut u8
    ) -> Result<Self, i32> {
        let tc = trans_color.map(|c| c.0 as i32).unwrap_or(VM_NO_TRANS_COLOR);
        
        let handle = unsafe { vm_graphic_create_layer_ex(x, y, width, height, tc, mode as i32, buffer) };
        if handle < 0 {
            return Err(handle);
        }

        unsafe { push_layer_handle(handle); }

        Ok(Self { handle, width, height })
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    #[inline]
    pub fn activate(&self) {
        unsafe { vm_graphic_active_layer(self.handle); }
    }

    #[inline]
    pub fn clear_bg(&self) {
        unsafe { vm_graphic_clear_layer_bg(self.handle); }
    }

    #[inline]
    pub fn set_opacity(&self, percent: u8) {
        let p = if percent > 100 { 100 } else { percent as i32 };
        unsafe { vm_graphic_set_layer_opacity(self.handle, p); }
    }

    #[inline]
    pub fn handle(&self) -> i32 {
        self.handle
    }

    #[inline]
    pub fn buffer_ptr(&self) -> *mut u8 {
        unsafe { vm_graphic_get_layer_buffer(self.handle) }
    }

    pub fn buffer_mut(&mut self) -> Option<&mut [Color]> {
        let ptr = self.buffer_ptr();
        
        if ptr.is_null() {
            return None;
        }
        let color_ptr = ptr as *mut Color;

        let len = (self.width * self.height) as usize;

        unsafe {
            Some(core::slice::from_raw_parts_mut(color_ptr, len))
        }
    }

    pub fn translate(&self, x: i32, y: i32) -> Result<(), i32> {
        let res = unsafe { vm_graphic_translate_layer(self.handle, x, y) };
        if res == 0 { Ok(()) } else { Err(res) }
    }

    pub fn resize(&mut self, width: i32, height: i32) -> Result<(), i32> {
        let res = unsafe { vm_graphic_resize_layer(self.handle, width, height) };
        if res == 0 { 
            self.width = width;
            self.height = height;
            Ok(()) 
        } else { 
            Err(res) 
        }
    }

    pub fn position(&self) -> Result<(i32, i32, i32, i32), i32> {
        let mut x = 0;
        let mut y = 0;
        let mut w = 0;
        let mut h = 0;
        let res = unsafe { 
            vm_graphic_get_layer_position(self.handle, &mut x, &mut y, &mut w, &mut h) 
        };
        if res == 0 { 
            Ok((x, y, w, h)) 
        } else { 
            Err(res) 
        }
    }

    pub fn enable_alpha_blending(&self) -> Result<(), i32> {
        let res = unsafe { vm_graphic_set_alpha_blending_layer(self.handle) };
        if res == 0 { Ok(()) } else { Err(res) }
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        unsafe { pop_layer_handle(self.handle); }
    }
}

pub fn flush(layers: &[&Layer]) {
    if layers.is_empty() { return; }
    
    let handles: Vec<i32> = layers.iter().map(|l| l.handle()).collect();
    
    unsafe {
        vm_graphic_flush_layer(handles.as_ptr(), handles.len() as i32);
    }
}

pub fn disable_alpha_blending() -> Result<(), i32> {
    let res = unsafe { vm_graphic_set_alpha_blending_layer(-1) };
    if res == 0 { Ok(()) } else { Err(res) }
}